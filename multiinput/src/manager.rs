use winapi::um::winuser::{CreateWindowExW, DefWindowProcW, WNDCLASSEXW, HWND_MESSAGE, CW_USEDEFAULT, RegisterClassExW}; 
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::UINT; 
use winapi::um::libloaderapi::GetModuleHandleW;
use event::RawEvent;
use devices::{Devices, JoystickState};
use rawinput::{get_joystick_state, get_event};
use registrar;
use std::time::{SystemTime, UNIX_EPOCH};

use std::ptr;
use std::mem;
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

use std::sync::mpsc::{
    Sender,
    Receiver,
    channel
};

enum Command {
    Register(DeviceType),
    GetEvent,
    GetJoystickState(usize),
    Finish,
    PrintDeviceList,
    //NOTE
    //added by Thoth Gunter 
    SendDeviceList,
    GetDeviceStats,
}

/// Types of Raw Input Device
#[derive(PartialEq, Eq, Clone)] 
pub enum DeviceType {
    Mice,
    Keyboards,
    Joysticks(XInputInclude),
}

/// Denotes if Xbox360 controllers should be used
/// Please Note: Rawinput support for official Xbox360 controllers
/// is very limited (triggers share same axis, no support for
/// rumble or the central X button)
/// Please see https://en.wikipedia.org/wiki/DirectInput#Xbox_360_Controller_support
/// for more details
#[derive(PartialEq, Eq, Clone)]
pub enum XInputInclude {
    True,
    False
}

#[derive(Default)]
pub struct DeviceStats {
    pub number_of_mice: usize,
    pub number_of_keyboards: usize,
    pub number_of_joysticks: usize,
}

/// Manages Raw Input Processing
pub struct RawInputManager {
    joiner: Option<JoinHandle<()>>,
    sender: Sender<Command>,
    receiver: Receiver<Option<RawEvent>>,
    joystick_receiver: Receiver<Option<JoystickState>>,
    device_stats_receiver: Receiver<DeviceStats>,
    device_names_receiver: Receiver<Vec<String>>,
}

impl RawInputManager {
    pub fn new() -> Result<RawInputManager, &'static str> {
        let (tx, rx) = channel();
        let (tx2, rx2) = channel();
        let (tx_joy, rx_joy) = channel();
        let (tx_stats, rx_stats) = channel();
        let (tx_device_name, rx_device_name) = channel();

        let builder = thread::Builder::new().name("multiinput".to_string());
        let joiner = builder.spawn(move || {
            let hwnd = setup_message_window();
            let mut event_queue = VecDeque::new();
            let mut devices = Devices::new();
            let mut exit = false;
            let mut registrar = registrar::RawInputRegistrar::new();
            while !exit {
                match  rx.recv().unwrap() {
                    Command::Register(thing) =>
                    {devices = registrar.register_devices(hwnd, thing).unwrap();
                        tx2.send(None).unwrap();},
                    Command::GetEvent =>
                        tx2.send(get_event(&mut event_queue, &mut devices)).unwrap(),
                    Command::Finish => {exit = true;},
                    Command::GetJoystickState(id) =>
                        tx_joy.send(get_joystick_state(&devices, id)).unwrap(),
                    Command::PrintDeviceList =>
                        print_raw_device_list(&devices),
                    //NOTE
                    //added by Thoth Gunter 
                    Command::SendDeviceList =>
                        tx_device_name.send(send_raw_device_list(&devices)).unwrap(),
                    Command::GetDeviceStats =>
                        tx_stats.send(get_device_stats(&devices)).unwrap(),
                };
            };
        }).unwrap();
        Ok(RawInputManager{
            joiner: Some(joiner),
            sender: tx,
            receiver: rx2,
            joystick_receiver: rx_joy,
            device_stats_receiver: rx_stats,
            device_names_receiver: rx_device_name,
        })
    }

    /// Allows Raw Input devices of type device_type to be received from the Input Manager
    pub fn register_devices(&mut self, device_type: DeviceType) {
        self.sender.send(Command::Register(device_type)).unwrap();
        self.receiver.recv().unwrap();
    }

    /// Get Event from the Input Manager
    pub fn get_event(&mut self) -> Option<RawEvent> {
        self.sender.send(Command::GetEvent).unwrap();
        self.receiver.recv().unwrap()
    }

    /// Get Joystick State from the Input Manager
    pub fn get_joystick_state(&mut self, id: usize) -> Option<JoystickState> {
        self.sender.send(Command::GetJoystickState(id)).unwrap();
        self.joystick_receiver.recv().unwrap()
    }

    /// Print List of Potential Input Devices
    pub fn print_device_list(& self) {
        self.sender.send(Command::PrintDeviceList).unwrap();
    }

    //NOTE
    //added my Thoth Gunter 
    /// send vec of list of Potential Input Devices
    pub fn send_device_list(& self)->Vec<String> {
        self.sender.send(Command::SendDeviceList).unwrap();
        self.device_names_receiver.recv().unwrap()
    }

    /// Get Device Stats (number of connected devices)
    pub fn get_device_stats(&self) -> DeviceStats{
        self.sender.send(Command::GetDeviceStats).unwrap();
        self.device_stats_receiver.recv().unwrap()
    }
}

impl Drop for RawInputManager {
    fn drop(&mut self) {
        self.sender.send(Command::Finish).unwrap();
        self.joiner.take().unwrap().join().unwrap();
    }
}

fn setup_message_window() -> HWND{
    let hwnd: HWND;
    unsafe{
        let hinstance = GetModuleHandleW(ptr::null());
        if hinstance == ptr::null_mut(){
            panic!("Instance Generation Failed");
        }

        let current_time = SystemTime::now();
        let classname_str = format!("RawInput Hidden Window {:?}", current_time.duration_since(UNIX_EPOCH).unwrap());

        let classname =
            OsStr::new(&classname_str).encode_wide().chain(Some(0).into_iter())
            .collect::<Vec<_>>();

        let wcex = WNDCLASSEXW{
            cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hbrBackground: ptr::null_mut(),
            hCursor:  ptr::null_mut(),
            hIcon:  ptr::null_mut(),
            hIconSm:  ptr::null_mut(),
            hInstance: hinstance,
            lpfnWndProc: Some(DefWindowProcW),
            lpszClassName: classname.as_ptr(),
            lpszMenuName: ptr::null_mut(),
            style: 0,
        };
        let a = RegisterClassExW(&wcex);
        if a == 0{
	        panic!("Registering WindowClass Failed!");
        }

        hwnd = CreateWindowExW(0,
                               classname.as_ptr(),
                               classname.as_ptr(),
                               0,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               CW_USEDEFAULT,
                               HWND_MESSAGE,
                               ptr::null_mut(),
                               hinstance,
                               ptr::null_mut());
        if hwnd.is_null(){
            panic!("Window Creation Failed!");
        }
    }
    hwnd
}

/// Prints a list of all available raw input devices
fn print_raw_device_list (devices: &Devices) {;
    println!("Mice:");
    for mouse in devices.mice.clone() {
        println!("{:?}", mouse.names);
        println!("{:?}", mouse.serial);
    }
    println!("Keyboards:");
    for keyboard in devices.keyboards.clone() {
        println!("{:?}", keyboard.names);
        println!("{:?}", keyboard.serial);
    }
    println!("Hids:");
    for joystick in devices.joysticks.clone() {
        println!("{:?}", joystick.names);
        println!("{:?}", joystick.serial);
    }
}

//NOTE
//added by Thoth Gunter 
fn send_raw_device_list (devices: &Devices)->Vec<String> {;
    let mut v = Vec::new();
    for (i, mouse) in devices.mice.clone().iter().enumerate() {
        v.push(format!("Mice{}_{:?}", i, mouse.names));
    }
    for (i, keyboard) in devices.keyboards.clone().iter().enumerate() {
        v.push(format!("Keyboard{}_{:?}", i, keyboard.names));
    }
    for (i, joystick) in devices.joysticks.clone().iter().enumerate() {
        v.push(format!("Joystick{}_{:?}", i, joystick.names));
    }
    return v;
}

fn get_device_stats(devices: &Devices) -> DeviceStats {
    DeviceStats {
        number_of_mice: devices.mice.len(),
        number_of_keyboards: devices.keyboards.len(),
        number_of_joysticks: devices.joysticks.len(),
    }
}
