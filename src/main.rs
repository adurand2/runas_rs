
use windows::core::PWSTR;
use windows::Win32::System::RemoteDesktop::WTSQueryUserToken;
use windows::Win32::System::Threading::{CreateProcessAsUserW, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_USESHOWWINDOW, STARTUPINFOW};
use windows::Win32::Security::{DuplicateTokenEx, SecurityImpersonation, TOKEN_ACCESS_MASK, TokenPrimary};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HANDLE;
use std::env;

fn str_to_pwstr(s: &str) -> PWSTR {
    // Convert &str to UTF-16 and append a null terminator
    let mut utf16: Vec<u16> = s.encode_utf16().chain(std::iter::once(0)).collect();

    // Get a mutable pointer to the buffer
    PWSTR(utf16.as_mut_ptr())
}


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        
        println!("give me the exe path");
        return;
    };


    let fullarglist: String = args.iter().skip(1).cloned().collect::<Vec<String>>().join(" ");
    unsafe{

        // Process creation flags are set to 0
        let dwcreationflags = PROCESS_CREATION_FLAGS(0);
        

        // Convert executable string to PWSTR
        let lpcommandline = str_to_pwstr(&fullarglist);


        // Hardcode sessionID to 1 for now
        let session_id = 2;

        // create handle pointers for the original user token and the duplicated one
        let mut old_handle: HANDLE = Default::default();
        let old_token : *mut HANDLE = &mut old_handle; 

        let mut new_handle: HANDLE = Default::default();
        let new_token : *mut HANDLE = &mut new_handle;



        // Try to retrieve the user token for the specified session
        let qut_result = WTSQueryUserToken(session_id, old_token);
        if let Err(e) = qut_result {

            println!("Error getting user token: {}",e);
            println!("{}", GetLastError().to_hresult());
            return;
        }
        
        // Duplicate the token
        let dte_result = DuplicateTokenEx(*old_token,TOKEN_ACCESS_MASK(0), None, SecurityImpersonation, TokenPrimary, new_token);
        if let Err(e) = dte_result {
            println!("Error duplicating token: {}",e);
            return;
        }

        // Output var for process information
        let mut lpprocessinformation: PROCESS_INFORMATION = Default::default();
        let lpprocessinformation_ptr : *mut PROCESS_INFORMATION = &mut lpprocessinformation; 

        // Startup options to ensure that the program renders on the GUI
        let mut lpstartupinfo = STARTUPINFOW{
            lpDesktop: str_to_pwstr("winsta0\\default"), // The default GUI desktop
            dwFlags: STARTF_USESHOWWINDOW,
            wShowWindow: 1,
            ..Default::default()
        };

        lpstartupinfo.cb = std::mem::size_of_val(&lpstartupinfo) as u32;
        
        // finally, create the process as the other user
        let create_process_result = CreateProcessAsUserW(Some(*new_token),None , Some(lpcommandline), None, None, false, dwcreationflags, None, None, &lpstartupinfo, lpprocessinformation_ptr);
        if let Err(e) = create_process_result {
            println!("Error creating process: {}",e);
            println!("{}", GetLastError().to_hresult());
            return;
        }

        println!("congrats on your new process!");
        return;
    }
}
