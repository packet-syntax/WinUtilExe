use colored::Colorize;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOW;
use windows::core::PCWSTR;

use windows::Win32::System::Console::{
    CONSOLE_MODE, ENABLE_VIRTUAL_TERMINAL_PROCESSING, GetStdHandle, STD_OUTPUT_HANDLE,
    SetConsoleMode,
};

fn main() {
    unsafe {
        if let Ok(handle) = GetStdHandle(STD_OUTPUT_HANDLE) {
            let mut mode = CONSOLE_MODE(0);
            if GetConsoleMode(handle, &mut mode).is_ok() {
                // Enable the virtual terminal processing flag for ANSI colors
                let _ = SetConsoleMode(
                    handle,
                    CONSOLE_MODE(mode.0 | ENABLE_VIRTUAL_TERMINAL_PROCESSING.0),
                );
            }
        }
    }

    colored::control::set_override(true);

    println!(
        "{}",
        "  ==================================================".bright_blue()
    );
    println!(
        "{}",
        "  Windows Utility Exe Launcher For Chris Titus WinUtil   "
            .blue()
            .bold()
    );
    println!("{}", "                    By $pa<ket".red().bold());
    println!(
        "{}",
        "  ==================================================".bright_blue()
    );
    println!(" ");

    println!("{}", "Select a option:".bold().cyan());
    println!("{}", "  1. Stable Branch".dimmed());
    println!("{}", "  2. Development Branch".dimmed());
    print!("\nSelect an option (1 or 2): ");

    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim();

    let powershell_cmd = match choice {
        "1" => "irm https://christitus.com/win | iex",
        "2" => "irm https://christitus.com/windev | iex",
        _ => {
            println!("Invalid selection. Defaulting to Stable Branch.");
            "irm https://christitus.com/win | iex"
        }
    };

    let arguments = format!(
        "-NoProfile -ExecutionPolicy Bypass -Command \"{}\"",
        powershell_cmd
    );

    let operation: Vec<u16> = OsStr::new("runas").encode_wide().chain(Some(0)).collect();
    let file: Vec<u16> = OsStr::new("powershell.exe")
        .encode_wide()
        .chain(Some(0))
        .collect();
    let parameters: Vec<u16> = OsStr::new(&arguments)
        .encode_wide()
        .chain(Some(0))
        .collect();

    unsafe {
        let result = ShellExecuteW(
            Some(HWND(ptr::null_mut())),
            PCWSTR(operation.as_ptr()),
            PCWSTR(file.as_ptr()),
            PCWSTR(parameters.as_ptr()),
            PCWSTR(ptr::null()),
            SW_SHOW,
        );

        if result.0 as usize > 32 {
            println!("\nUAC accepted. Opening requested branch...");
        } else {
            eprintln!(
                "\nUAC denied or failed to launch. Error code: {}",
                result.0 as usize
            );
        }
    }
}

use windows::Win32::System::Console::GetConsoleMode;
