
#![feature(vec_into_raw_parts)]
#![feature(new_uninit)]
#![feature(maybe_uninit_ref)]
extern crate winapi;

use std::mem::{size_of, MaybeUninit};

use winapi::shared::minwindef::{DWORD, FALSE, FILETIME, HMODULE, TRUE};
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetProcessTimes, GetSystemTimes, OpenProcess};
use winapi::um::psapi::{EnumProcessModules, EnumProcesses, GetModuleBaseNameA};
use winapi::um::winnt::{CHAR, HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

const MAX_FILENAME: usize = 260;

//#[cfg(windows)] extern crate winapi;
use std::io::Error;
use std::thread::sleep;
use std::time::{Duration};
use std::convert::TryInto;

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

use winapi::um::winuser::MOUSEEVENTF_ABSOLUTE;
use winapi::um::winuser::MOUSEEVENTF_LEFTUP;
use winapi::um::winuser::MOUSEEVENTF_LEFTDOWN;
use winapi::um::winuser::MOUSEEVENTF_RIGHTDOWN;
use winapi::um::winuser::MOUSEEVENTF_RIGHTUP;
use winapi::um::winuser::MOUSEEVENTF_MOVE;

// Проблема в камере в игре, когда она наклоняется центр экрана это земля
// Когда персонаж прямо смотрит, координаты (1300,1000) это центр (0,0)
fn main(){
    test_pos_game();
}

fn test_pos_game(){
    let mut point = winapi::shared::windef::POINT{x:0,y:0};
    let mut lpPoint:winapi::shared::windef::LPPOINT = &mut point;
    loop{
        sleep(Duration::from_millis(500));
        show_lpPoint(lpPoint);

       // Отловить нажатие мыши
        unsafe {
            if  winapi::um::winuser::GetKeyState(1) == -127 || winapi::um::winuser::GetKeyState(1) == -128{
                println!("Mouse left click {}",winapi::um::winuser::GetKeyState(1));
            }
            if  winapi::um::winuser::GetKeyState(2) == -127 || winapi::um::winuser::GetKeyState(2) == -128{
                println!("Mouse right click {}",winapi::um::winuser::GetKeyState(2));
            }
        }
    }
}

fn test_fire_game(){
    let mut point = winapi::shared::windef::POINT{x:0,y:0};
    let mut lpPoint:winapi::shared::windef::LPPOINT = &mut point;

    unsafe {
        winapi::um::winuser::SetCursorPos(0, 0);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
        sleep(Duration::from_millis(1000));

        let TEST_X = 1300;
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(TEST_X+get_x_err_abs(TEST_X)),get_pos_y_abs(0), 0, 0);
        show_lpPoint(lpPoint);
        fire();

        sleep(Duration::from_millis(1000));

        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(TEST_X+get_x_err_abs(TEST_X)),get_pos_y_abs(0), 0, 0);
        show_lpPoint(lpPoint);
        fire();


        sleep(Duration::from_millis(1000));

        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(TEST_X+get_x_err_abs(TEST_X)),get_pos_y_abs(0), 0, 0);
        show_lpPoint(lpPoint);
        fire();





        /*
        //winapi::um::winuser::SetCursorPos(470, 510);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(470+get_x_err_abs(470)),get_pos_y_abs(510), 0, 0);
        fire();
        sleep(Duration::from_millis(1000));
        //draw_point_relative();

        //winapi::um::winuser::SetCursorPos(1810, 1180);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(1810+get_x_err_abs(1810)),get_pos_y_abs(1180), 0, 0);
        fire();
        sleep(Duration::from_millis(1000));
        //draw_point_relative();

        //winapi::um::winuser::SetCursorPos(2201, 730);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(2201+get_x_err_abs(2201)),get_pos_y_abs(730), 0, 0);
        fire();
        sleep(Duration::from_millis(1000));
        //draw_point_relative();
        */
    }
}




fn test()  {
//   print_message("Hello, world!").unwrap();

// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-mouse_event
// https://docs.rs/winapi/0.3.8/winapi/um/winuser/index.html#functions

    let mut spx:u32 = 100;// горизонтальная ось
    let mut spy:u32 = 20;//
    let sensitivity = 3;

    let direction_vector_step_x:u32 = 2;// mickey
    let direction_vector_step_y:u32 = 10;// mickey

    let mut rect: winapi::shared::windef::RECT = winapi::shared::windef::RECT{left:0,top:0,right:3072,bottom:1705};

    let pgui: *mut winapi::um::winuser::GUITHREADINFO = std::ptr::null_mut();
    let mut count:i8 = 3;



    unsafe {

        /*
                let lpcbNeeded: *mut u32 = std::ptr::null_mut();

                let mut buffer = vec![0_u32; 1000];
                let s = buffer.as_mut_slice();
                let buffer_ptr = s.as_mut_ptr();

                //let buffer = &mut [0_u32,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
               // let buffer_ptr = buffer.as_mut_ptr();
                let mut cb:u32 = 4000;//(buffer.len() * std::mem::size_of::<u32>()) as u32;// Размер массива pProcessIds в байтах

               if winapi::um::psapi::EnumProcesses(buffer_ptr,cb,lpcbNeeded) ==0 {
                   println!("Error code:{:?}",  kernel32::GetLastError());
               }else{
                   for i in 0..buffer.len() {
                       let pid = &*buffer_ptr.offset(i as isize);
                       if *pid!=0 {
                           println!("{:?}",*pid );
                       }else{
                           println!("Error code:{:?}",  kernel32::GetLastError());
                       }
                   }
               }
        */


        //winapi::um::winuser::GetClientRect(hwndMain, &rcClient);




        //let HDESK = winapi::um::winuser::OpenInputDesktop(0x0001, 1, winapi::um::winnt::GENERIC_ALL);
        //println!("EnableMouseInPointer={}", winapi::um::winuser::EnableMouseInPointer(1));
        //winapi::um::winuser::SetThreadDesktop(HDESK);





        //winapi::um::winuser::ReleaseCapture();// отпустить мышь
        // X:3072, Y:1705
        // X:65535u32-100 Y:65535u32-200
        let mut point = winapi::shared::windef::POINT{x:0,y:0};
        let mut lpPoint:winapi::shared::windef::LPPOINT = &mut point;


if false {
            let mut i = 0;
            let mut temp_x = 0;
            let mut pos_x = 30;
            winapi::um::winuser::SetCursorPos(0, 0);
            let mut err = 0;
            while(i<50){
                i+=1;//err+=5;
                winapi::um::winuser::SetCursorPos(pos_x, 730);// абсолютно
                winapi::um::winuser::GetCursorPos(lpPoint);
                temp_x = (*lpPoint).x;

                winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(pos_x+get_x_err_abs(pos_x)),get_pos_y_abs(730), 0, 0);
                winapi::um::winuser::GetCursorPos(lpPoint);
                println!("pos_x = {}  SetCursorPos = {} mouse_event = {} def={} err={}",pos_x,temp_x,(*lpPoint).x,temp_x-(*lpPoint).x,err);
                pos_x+=61;
                if pos_x > 3072{
                    break;
                }
            }
        }




        winapi::um::winuser::SetCursorPos(0, 0);
        winapi::um::winuser::SetCursorPos(470, 510);// абсолютно
        draw_point_relative();
        show_lpPoint(lpPoint);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(470+get_x_err_abs(470)),get_pos_y_abs(510), 0, 0);
        draw_point_relative();
        show_lpPoint(lpPoint);

        winapi::um::winuser::SetCursorPos(1810, 1180);// абсолютно
        draw_point_relative();
        show_lpPoint(lpPoint);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(1810+get_x_err_abs(1810)),get_pos_y_abs(1180), 0, 0);
        draw_point_relative();
        show_lpPoint(lpPoint);


        winapi::um::winuser::SetCursorPos(2201, 730);// абсолютно
        draw_point_relative();
        show_lpPoint(lpPoint);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE, get_pos_x_abs(2201+get_x_err_abs(2201)),get_pos_y_abs(730), 0, 0);
        draw_point_relative();
        show_lpPoint(lpPoint);

        winapi::um::winuser::ReleaseCapture();// отпустить мышь



        if(false) {
            // loop {
            spx = 100;
            spy = 20;

            for _ in 0..sensitivity {
                // winapi::um::winuser::SetPhysicalCursorPos(x, x * 2);
                // MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_MOVE
                winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE, spx, spy, 0, 0);

                draw_point_relative();

                //sleep(Duration::from_millis(100));
                show_lpPoint(lpPoint);
            }


            /* winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE, 0u32,  50u32, 0, 0);

             draw_point_relative();
             winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE, 50u32,  0u32, 0, 0);

             draw_point_relative();*/

            //winapi::um::winuser::mouse_event(0x0002, x.try_into().unwrap(),  (x * 2).try_into().unwrap(), 0, 0);
            //winapi::um::winuser::mouse_event(0x0004, x.try_into().unwrap(),  (x * 2).try_into().unwrap(), 0, 0);

            // Удар имитация нажатия левой кнопки мыши
            //winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            //winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);

            // Прицел имитация нажатия правой кнопки мыши
            //winapi::um::winuser::mouse_event(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
            //winapi::um::winuser::mouse_event(MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);

            // fire();

            //  winapi::um::winuser::mouse_event(0x8000, 100, 400, 0, 0);

            //winapi::um::winuser::CloseDesktop(HDESK);
            //break;
            sleep(Duration::new(3, 0));
            count-=1;
            if count == 0 {
                // break;
            }
            println!("count={}",count);
            //}
        }

        if(false){
            // Открыть Paint в верхнем левом углу
            // старт
            //winapi::um::winuser::SetPhysicalCursorPos(400, 400);// абсолютно
            //winapi::um::winuser::SetCaretPos(400, 400);// относительно текущей позиции курсора, но один раз
            winapi::um::winuser::SetCursorPos(400, 400);// абсолютно
            draw_point_relative();


            // Первая точка справа
            winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE, 50u32,  0u32, 0, 0);
            draw_point_relative();

            // Вторая точка под первой
            winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE,0u32,  50u32, 0, 0);
            winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            draw_point_relative();

            // Третья точка на ровне с первой по вертикали но дальше по горизонтали
            //winapi::um::winuser::SetPhysicalCursorPos(500, 400);
            winapi::um::winuser::SetCursorPos(600, 400);
            {
                // winapi::um::winuser::SetCaretPos(50, -50);
                // winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE,0u32,  50u32, 0, 0);
            }
            draw_point_relative();
            winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
            winapi::um::winuser::mouse_event(MOUSEEVENTF_MOVE, 0u32,  50u32, 0, 0);
            winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
            //winapi::um::winuser::SetCursorPos(600, 400+204);// 50 = 204
            draw_point_relative();
        }


    }

}
fn get_x_err_abs(pos_x_rel:i32)->i32{
    pos_x_rel/(3072/50)
}
fn get_pos_x_abs(pos_x_rel:i32)->u32{
    (pos_x_rel*(65435/3072)) as u32
}
fn get_pos_y_abs(pos_y_rel:i32)->u32{
    (pos_y_rel*(65335/1705)) as u32
}
fn get_pos_x_rel(pos_x_abs:i32)->u32{
    (pos_x_abs/(65435/3072)) as u32
}
fn get_pos_y_rel(pos_y_abs:i32)->u32{
    (pos_y_abs/(65335/1705)) as u32
}

unsafe fn fire(){
    // Полный выстрел
    winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_RIGHTDOWN, 0, 0, 0, 0);
    //sleep(Duration::from_millis(800));
    //winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTDOWN, 0, 0, 0, 0);
    //winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTUP, 0, 0, 0, 0);
    sleep(Duration::from_millis(800));
    winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_RIGHTUP, 0, 0, 0, 0);
}

/**
 * Ставит точку там где находится курсор
 */
fn draw_point_relative(){
    unsafe{
        winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTDOWN,  0,  0, 0, 0);
        winapi::um::winuser::mouse_event(MOUSEEVENTF_LEFTUP, 0,  0, 0, 0);
    }
}
unsafe fn draw_point_abs(){
    winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTDOWN,  0,  0, 0, 0);
    winapi::um::winuser::mouse_event(MOUSEEVENTF_ABSOLUTE|MOUSEEVENTF_LEFTUP, 0,  0, 0, 0);
}

fn show_lpPoint(lpPoint:winapi::shared::windef::LPPOINT){
    unsafe{
        winapi::um::winuser::GetCursorPos(lpPoint);
        println!("GetCursorPos x={:?} y={:?}",(*lpPoint).x,(*lpPoint).y);
    }
}

unsafe fn show_rect(HWND:winapi::shared::windef::HWND,rect: winapi::shared::windef::LPRECT){
    if(winapi::um::winuser::GetClientRect(HWND,rect) != 0){
        println!("left={} top={} right={} bottom={}\n",(*rect).left,(*rect).top,(*rect).right,(*rect).bottom);
    }else{
        // https://docs.microsoft.com/en-us/windows/win32/debug/system-error-codes
        println!("Error code:{:?}\n",  kernel32::GetLastError());
    }

}

/**
    let result = get_process_ids(1000).unwrap();
    for i_u32 in result {
        println!("pid = {}", i);
    }
    // https://gist.github.com/F3real/47aaab6736902c6e25a7546af5142b46
*/
fn get_process_ids(capacity: usize) -> Result<Vec<DWORD>, u32> {
    let v = Vec::<DWORD>::with_capacity(capacity);
    let (ptr, _, _) = v.into_raw_parts();
    let mut lpcb_needed: DWORD = 0;

    unsafe {
        // EnumProcesses should never write more then given capacity
        if EnumProcesses(
            ptr as *mut DWORD,
            (size_of::<DWORD>() * capacity) as DWORD,
            &mut lpcb_needed,
        ) == TRUE
        {
            let mut ids =
                Vec::from_raw_parts(ptr, lpcb_needed as usize / size_of::<DWORD>(), capacity);
            ids.resize_with(lpcb_needed as usize / size_of::<DWORD>(), Default::default);
            Ok(ids)
        } else {
            Err(1)
        }
    }
}
/**
let pid:u32 = 10204;
let handle = get_process_handle(pid).unwrap();
*/
fn get_process_handle(pid: DWORD) -> Option<HANDLE> {
    let p_handle = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid) };
    if p_handle.is_null() {
        None
    } else {
        Some(p_handle)
    }
}

/**
let pid:u32 = 10204;
let handle = get_process_handle(pid).unwrap();
println!("process name:{}",get_process_name(handle).unwrap());
*/
fn get_process_name(p_handle: HANDLE) -> Result<String, u32> {
    let mut lph_module = Box::<HMODULE>::new_uninit();
    let mut cb_needed: DWORD = 0;
    unsafe {
        // Should return base module
        if EnumProcessModules(
            p_handle,
            lph_module.as_mut_ptr(),
            size_of::<HMODULE>() as u32,
            &mut cb_needed,
        ) != 0
        {
            let lph_main_module = lph_module.assume_init();
            let v: Vec<CHAR> = Vec::with_capacity(MAX_FILENAME);
            let (ptr, _, _) = v.into_raw_parts();
            let name_len =
                GetModuleBaseNameA(p_handle, *lph_main_module, ptr, MAX_FILENAME as u32) as usize;
            let mut name: Vec<u8> = Vec::from_raw_parts(ptr as *mut u8, name_len, MAX_FILENAME);
            if name_len > 0 {
                if name_len < MAX_FILENAME {
                    name.resize_with(name_len, Default::default);
                }
                return Ok(String::from_utf8(name).unwrap());
            }
        }
        Err(1)
    }
}









/*
        if let val =  winapi::um::winuser::GetSystemMetrics( winapi::um::winuser::SM_CXFULLSCREEN ) {
            println!("Ширина клиентской области для полноэкранного окна на основном мониторе {:?} пикселей\n",  val);
       }else{
           println!("Error code:{:?}\n",  kernel32::GetLastError());
       }
        if let val =  winapi::um::winuser::GetSystemMetrics( winapi::um::winuser::SM_CYFULLSCREEN ) {
            println!("Высота клиентской области для полноэкранного окна на основном мониторе {:?} пикселей\n",  val);
        }else{
            println!("Error code:{:?}\n",  kernel32::GetLastError());
        }
 */