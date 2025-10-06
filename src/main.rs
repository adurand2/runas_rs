use windows::core::PWSTR;
use windows::Win32::System::RemoteDesktop::WTSQueryUserToken;
use windows::Win32::System::Threading::{CreateProcessAsUserW, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTF_USESHOWWINDOW, STARTUPINFOW};
use windows::Win32::Security::{DuplicateTokenEx, SecurityImpersonation, TOKEN_ACCESS_MASK, TokenPrimary};
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};
use windows::Win32::System::RemoteDesktop::WTSGetActiveConsoleSessionId;
use std::env;

fn str_to_pwstr(input: &str) -> *mut u16 {
    // Convert the &str (UTF-8) to a Vec<u16> (UTF-16)
    let utf16: Vec<u16> = input.encode_utf16().collect();

    // We need to append a null terminator to the end of the UTF-16 encoded string
    let mut utf16_with_null = utf16;
    utf16_with_null.push(0); // Null-terminate the string

    // Convert the Vec<u16> into a raw pointer (equivalent to PWSTR)
    let ptr = utf16_with_null.as_mut_ptr();

    // Ensure the Vec doesn't get dropped before we're done using the pointer
    std::mem::forget(utf16_with_null);

    // Return the raw pointer as PWSTR (which is *mut u16)
    ptr
}

fn runProcess(lpcommandline: PWSTR, session_id: u32 ){

    unsafe {

        // Process creation flags are set to 0
        let dwcreationflags = PROCESS_CREATION_FLAGS(0);

        /*
            MessageBoxW(
                None,
                lpcommandline,           // Message
                lpcommandline,           // Title
                MB_OK,           // OK button
            );
        */

   
           // create handle pointers for the original user token and the duplicated one
           let mut old_handle: HANDLE = Default::default();
           let old_token : *mut HANDLE = &mut old_handle; 
   
           let mut new_handle: HANDLE = Default::default();
           let new_token : *mut HANDLE = &mut new_handle;
   
   
           // find the session ID of current user
           //let session_id = WTSGetActiveConsoleSessionId();
           //println!("running in session {}", session_id);
   
   
           // Try to retrieve the user token for the specified session
           let qut_result = WTSQueryUserToken(session_id, old_token);
           if let Err(e) = qut_result {
   
               println!("Error getting user token. You probably need to escalate to system: {}",e);
               //println!("{}", GetLastError().to_hresult());
               return;
           }
           
           // Duplicate the token
           let dte_result = DuplicateTokenEx(*old_token,TOKEN_ACCESS_MASK(0), None, SecurityImpersonation, TokenPrimary, new_token);
           if let Err(e) = dte_result {
               //println!("Error duplicating token: {}",e);
               return;
           }
   
           // Output var for process information
           let mut lpprocessinformation: PROCESS_INFORMATION = Default::default();
           let lpprocessinformation_ptr : *mut PROCESS_INFORMATION = &mut lpprocessinformation; 
   
           // Startup options to ensure that the program renders on the GUI
           let mut lpstartupinfo = STARTUPINFOW{
               lpDesktop: PWSTR(str_to_pwstr("winsta0\\default")), // The default GUI desktop
               dwFlags: STARTF_USESHOWWINDOW,
               wShowWindow: 1,
               ..Default::default()
           };
   
           lpstartupinfo.cb = std::mem::size_of_val(&lpstartupinfo) as u32;
           
           // finally, create the process as the other user
           let create_process_result = CreateProcessAsUserW(Some(*new_token), None , Some(lpcommandline) , None, None, false, dwcreationflags, None, None, &lpstartupinfo, lpprocessinformation_ptr);
           if let Err(e) = create_process_result {
               //println!("Error creating process: {}",e);
               //println!("{}", GetLastError().to_hresult());
               return;
           }
   
           println!("Process Started!");
           return;
       }
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        
        println!("give me the session (0 means all) and command to run");
        return;
    };

    let session_id: u32 = args[1].parse().unwrap_or(0);

    // horrifying code to join all args apart from the first into a single string
    let exePath: String = args.iter().skip(2).cloned().collect::<Vec<String>>().join(" ");

    let exePath = format!("\"{}\"", exePath);
    //println!("{}", fullarglist);


    unsafe{
        
        // Convert executable string to PWSTR
        let lpcommandline = PWSTR(str_to_pwstr(&exePath));

        // if session id is not 0, just run on that session id
        if session_id != 0 {
            runProcess(lpcommandline, session_id);
            return;
        }

        // otherwise go crazy
        let session_id = 0;
        runProcess(lpcommandline, session_id);

        let session_id = 1;
        runProcess(lpcommandline, session_id);

        let session_id = 2;
        runProcess(lpcommandline, session_id);

        let session_id = 3;
        runProcess(lpcommandline, session_id);

        let session_id = 4; 
        runProcess(lpcommandline, session_id);

        
        
    }
}
