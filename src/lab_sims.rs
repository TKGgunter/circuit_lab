use crate::rendertools::*;
use crate::inputhandler::*;
use crate::{WindowCanvas, OsPackage, SETICON, set_icon};
use crate::stb_tt_sys::*;
use crate::stb_image_sys::{StbiImage, stbi_load_from_memory_32bit};
use crate::miniz;
use crate::{FONT_NOTOSANS, FONT_NOTOSANS_BOLD};
use crate::misc::*;
use crate::ui_tools;
use matrixmath::*;


use std::io::prelude::*;
use std::ptr::{null, null_mut};

use std::f32::consts::PI;


use std::thread::sleep;
use std::time::{Instant, Duration};

use rand::thread_rng;
use rand_distr::{Normal, Distribution};

use crate::eq_potential::*;
use parser::*;


const WINDOW_WIDTH  : i32 = 1250;
const WINDOW_HEIGHT : i32 = 750;

const IMG_RESISTOR       : &[u8] = std::include_bytes!("../assets/resistor.bmp");
const IMG_BATTERY        : &[u8] = std::include_bytes!("../assets/battery.bmp");
const IMG_VOLTMETER      : &[u8] = std::include_bytes!("../assets/voltmeter.bmp");
const IMG_AMMETER        : &[u8] = std::include_bytes!("../assets/ammeter.bmp");
const IMG_CAPACITOR      : &[u8] = std::include_bytes!("../assets/capacitor.bmp");
const IMG_INDUCTOR       : &[u8] = std::include_bytes!("../assets/inductor.bmp");
const IMG_SWITCH_OPEN    : &[u8] = std::include_bytes!("../assets/switch_open.bmp");
const IMG_SWITCH_CLOSED  : &[u8] = std::include_bytes!("../assets/switch_closed.bmp");
const IMG_AC             : &[u8] = std::include_bytes!("../assets/ac.bmp");
const IMG_WIRE           : &[u8] = std::include_bytes!("../assets/wire.bmp");
const IMG_CUSTOM         : &[u8] = std::include_bytes!("../assets/custom_circuit_element.bmp"); 

const IMG_ICON           : &[u8] = std::include_bytes!("../assets/sdfviewer.bmp"); //TODO this is temp 


const IMG_ARROW          : &[u8] = std::include_bytes!("../assets/arrow.bmp");
const IMG_SCREENSHOT     : &[u8] = std::include_bytes!("../assets/screenshot_icon.bmp");
const IMG_SAVE           : &[u8] = std::include_bytes!("../assets/save.bmp");
const IMG_SAVE_ALT       : &[u8] = std::include_bytes!("../assets/save_alt.bmp");


const TA_FILE_NAME : &str = "TA.txt";
const CUSTOM_FILE_NAME : &str = "custom.cce";
const CIRCUIT_FILE_NAME : &str = "circuit_worksheet.exp";
const TIME_STEP: f32 = 0.006;//TODO this should be frame rate independent

const MAX_SAVED_MEASURMENTS : usize = 4000;
const FONT_MERRIWEATHER_LIGHT : &[u8] = std::include_bytes!("../assets/Merriweather-Light.ttf");

const PROPERTIES_W : i32 = 280; 
const PROPERTIES_H : i32 = 260; 
static mut GLOBAL_PROPERTIES_Z: usize = 0;

const DEFAULT_MESSAGE_ONSCREEN_DURATION : Duration = Duration::from_millis(5100);
const ERROR_MESSAGE_ONSCREEN_DURATION   : Duration = Duration::from_millis(7500);



fn set_global_properties_z(x: usize){unsafe{
    GLOBAL_PROPERTIES_Z = x;
}}
fn get_and_update_global_properties_z()->usize{unsafe{
    let rt = GLOBAL_PROPERTIES_Z;
    GLOBAL_PROPERTIES_Z += 1;
    return rt;
}}
fn get_global_properties_z()->usize{unsafe{
    let rt = GLOBAL_PROPERTIES_Z;
    return rt;
}}




const DEFAULT_CUSTOM_ELEMENTS : [CircuitElement ; 4] = { 
    let mut resistor_1 = CircuitElement::empty(); 

    {
        resistor_1.circuit_element_type = SelectedCircuitElement::Custom;
        resistor_1.resistance = 5f32;

        resistor_1.label = TinyString::new();
        resistor_1.label.buffer[0] = 'R' as u8;
        resistor_1.label.buffer[1] = 'e' as u8;
        resistor_1.label.buffer[2] = 's' as u8;
        resistor_1.label.buffer[3] = 'i' as u8;
        resistor_1.label.buffer[4] = 's' as u8;
        resistor_1.label.buffer[5] = 't' as u8;
        resistor_1.label.buffer[6] = 'o' as u8;
        resistor_1.label.buffer[7] = 'r' as u8;
        resistor_1.label.buffer[8] = '_' as u8;
        resistor_1.label.buffer[9] = '1' as u8;
        resistor_1.label.cursor = 10;
    }


    let mut resistor_2 = CircuitElement::empty(); 
    {
        resistor_2.circuit_element_type = SelectedCircuitElement::Custom;
        resistor_2.resistance = 2.0f32;
        resistor_2.unc_resistance = 0.5f32;
        resistor_2.label = TinyString::new();
        resistor_2.label.buffer[0] = 'R' as u8;
        resistor_2.label.buffer[1] = '2' as u8;
        resistor_2.label.buffer[2] = 'v' as u8;
        resistor_2.label.buffer[3] = '1' as u8;
        resistor_2.label.cursor = 4;
    }

    let mut resistor_3 = CircuitElement::empty(); 
    {
        resistor_3.circuit_element_type = SelectedCircuitElement::Custom;
        resistor_3.resistance = 2.1f32;
        resistor_3.unc_resistance = 0.01f32;
        resistor_3.label = TinyString::new();
        resistor_3.label.buffer[0] = 'R' as u8;
        resistor_3.label.buffer[1] = '2' as u8;
        resistor_3.label.buffer[2] = 'v' as u8;
        resistor_3.label.buffer[3] = '2' as u8;
        resistor_3.label.cursor = 4;
    }

    let mut resistor_4 = CircuitElement::empty(); 
    {
        resistor_4.circuit_element_type = SelectedCircuitElement::Custom;
        resistor_4.resistance = 5.3f32;
        resistor_4.unc_resistance = 0.25f32;
        resistor_4.label = TinyString::new();
        resistor_4.label.buffer[0] = 'R' as u8;
        resistor_4.label.buffer[1] = '_' as u8;
        resistor_4.label.buffer[2] = '5' as u8;
        resistor_4.label.buffer[3] = 'o' as u8;
        resistor_4.label.buffer[4] = 'h' as u8;
        resistor_4.label.buffer[5] = 'm' as u8;
        resistor_4.label.buffer[6] = '?' as u8;
        resistor_4.label.cursor = 7;
    }

    [resistor_4, resistor_1, resistor_2, resistor_3, ] 
};




//TODO Color should be capped
pub const COLOR_BKG        : [f32; 4] = [81.0/255.0, 33.0/255.0, 1.0, 1.0];
pub const COLOR_MENU_BKG   : [f32; 4] = [97.0/255.0, 163.0/255.0, 1.0, 1.0];
pub const COLOR_GRID       : [f32; 4] = [101.0/255.0, 53.0/255.0, 1.0, 1.0];
pub const COLOR_TEXT       : [f32; 4] = C4_WHITE;
pub const COLOR_TEXT_SOLV  : [f32; 4] = C4_YELLOW;

pub const COLOR_PROPERTY_BKG1  : [f32; 4] = C4_BLACK;
pub const COLOR_PROPERTY_BKG2  : [f32; 4] = [0.12, 0.12, 0.12, 1.0f32];
pub const COLOR_PROPERTY_MOVE1  : [f32; 4] = [0.23, 0.05, 0.23, 1.0f32];
pub const COLOR_PROPERTY_MOVE2  : [f32; 4] = [0.3, 0.12, 0.3, 1.0f32];
//
//pub const COLOR_BUTTON  : [f32; 4] = C4_WHITE;
//pub const COLOR_BUTTON_TEXT  : [f32; 4] = C4_WHITE;

const GRID_SIZE: i32 = 20;

const DEFAULT_RESISTANCE   : f32 = 2.0;
const DEFAULT_VOLTAGE      : f32 = 2.0;
const DEFAULT_CAPACITANCE  : f32 = 2.0;
const DEFAULT_INDUCTANCE   : f32 = 2.0;
const DEFAULT_FREQUENCY    : f32 = 0.5;
const VOLTMETER_RESISTANCE : f32 = 99999.0;
const WIRE_RESISTANCE      : f32 = 0.000001;


const PANEL_FONT : f32 = 23.0;


#[derive(PartialEq, Copy, Clone, Debug)]
enum SelectedCircuitElement{
    Resistor,
    Wire,
    Battery,
    Capacitor,
    Inductor,
    Voltmeter,
    Ammeter,
    Switch,
    AC,
    Custom,
    None,
    CustomVoltmeter,
    CustomAmmeter,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum ACSourceType{
    Sin,
    Step,
}

static mut unique_node_int: usize = 0;
fn get_unique_id()->usize{unsafe{ //NOTE this is NOT thread safe
    let id = unique_node_int;

    unique_node_int += 1;
    return id;

}}

fn set_unique_id(n: usize){unsafe{
    unique_node_int = n;
}}

#[derive(PartialEq, Copy, Clone, Debug)]
enum CircuitElementDirection{
    AtoB,
    BtoA,
}

struct CircuitElementTextBox{
    resistance_textbox  : TextBox,
    voltage_textbox     : TextBox,
    current_textbox     : TextBox,
    capacitance_textbox : TextBox,
    inductance_textbox  : TextBox,
    charge_textbox      : TextBox,
    magflux_textbox     : TextBox,
    max_voltage_textbox : TextBox,
    frequency_textbox   : TextBox,
    label_textbox       : TextBox,

    unc_resistance_textbox  : TextBox,
    unc_voltage_textbox     : TextBox,
    unc_current_textbox     : TextBox,
    unc_capacitance_textbox : TextBox,
    unc_inductance_textbox  : TextBox,
    unc_charge_textbox      : TextBox,
    unc_magflux_textbox     : TextBox,

    bias_textbox      : TextBox,
    noise_textbox     : TextBox,
    drift_textbox     : TextBox,
}

impl CircuitElementTextBox{
    fn new()->CircuitElementTextBox{
        fn fn_textbox()->TextBox{
            let mut tb = TextBox::new();
            tb.max_char = 6;
            tb.text_size = PANEL_FONT;
            tb.max_render_length = get_advance_string(&"X".repeat(tb.max_char as _), PANEL_FONT);
            tb
        }

        let mut label = TextBox::new();
        label.text_size = PANEL_FONT;
        label.max_char = _FIXED_CHAR_BUFFER_SIZE as i32;
        label.max_render_length = get_advance_string(&"X".repeat(label.max_char as _), PANEL_FONT);

        CircuitElementTextBox{
            resistance_textbox  : fn_textbox(),
            voltage_textbox     : fn_textbox(),
            current_textbox     : fn_textbox(),
            capacitance_textbox : fn_textbox(),
            inductance_textbox  : fn_textbox(),
            charge_textbox      : fn_textbox(),
            magflux_textbox     : fn_textbox(),
            max_voltage_textbox : fn_textbox(),
            frequency_textbox   : fn_textbox(),
            label_textbox       : label,

            unc_resistance_textbox  : fn_textbox(),
            unc_voltage_textbox     : fn_textbox(),
            unc_current_textbox     : fn_textbox(),
            unc_capacitance_textbox : fn_textbox(),
            unc_inductance_textbox  : fn_textbox(),
            unc_charge_textbox      : fn_textbox(),
            unc_magflux_textbox     : fn_textbox(),

            bias_textbox      : fn_textbox(),
            noise_textbox     : fn_textbox(),
            drift_textbox     : fn_textbox(),
        }
    }
    fn new_guess_length()->CircuitElementTextBox{
        fn fn_textbox()->TextBox{
            let mut tb = TextBox::new();
            tb.max_char = 6;
            tb.text_size = PANEL_FONT;
            tb.max_render_length = (tb.max_char as f32 * PANEL_FONT/2f32) as i32;
            tb
        }

        let mut label = TextBox::new();
        label.text_size = PANEL_FONT;
        label.max_char = _FIXED_CHAR_BUFFER_SIZE as i32;
        label.max_render_length = (label.max_char as f32 * PANEL_FONT/2f32) as i32;

        CircuitElementTextBox{
            resistance_textbox  : fn_textbox(),
            voltage_textbox     : fn_textbox(),
            current_textbox     : fn_textbox(),
            capacitance_textbox : fn_textbox(),
            inductance_textbox  : fn_textbox(),
            charge_textbox      : fn_textbox(),
            magflux_textbox     : fn_textbox(),

            max_voltage_textbox : fn_textbox(),
            frequency_textbox   : fn_textbox(),

            label_textbox       : label,

            unc_resistance_textbox  : fn_textbox(),
            unc_voltage_textbox     : fn_textbox(),
            unc_current_textbox     : fn_textbox(),
            unc_capacitance_textbox : fn_textbox(),
            unc_inductance_textbox  : fn_textbox(),
            unc_charge_textbox      : fn_textbox(),
            unc_magflux_textbox     : fn_textbox(),

            bias_textbox      : fn_textbox(),
            noise_textbox     : fn_textbox(),
            drift_textbox     : fn_textbox(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CircuitElement{
    circuit_element_type: SelectedCircuitElement, //Special
    orientation: f32,                             //Special
    x: i32,                                       //Special
    y: i32,                                       //Special
    length: i32,                                  //Special
    selected: bool,
    selected_rotation: bool,

    properties_selected: bool,
    properties_move_selected: bool,
    properties_offset_x: Option<i32>,
    properties_offset_y: Option<i32>,


    unique_a_node: usize,                        //Special
    a_node: usize,
    unique_b_node: usize,                        //Special
    b_node: usize,

    resistance: f32,                            //Special
    voltage: f32,                               //Special
    current: f32,                               //Special

    unc_resistance: f32,                            //Special
    unc_voltage: f32,                               //Special
    unc_current: f32,                               //Special

    capacitance: f32,                           //Special
    inductance: f32,                            //Special
    charge:     f32,                            //Special
    magnetic_flux: f32,                         //Special

    unc_capacitance: f32,                           //Special
    unc_inductance: f32,                            //Special
    unc_charge:     f32,                            //Special
    unc_magnetic_flux: f32,                         //Special
    
    max_voltage: f32,
    d_voltage_over_dt: f32,
    frequency: f32,

    solved_voltage: Option<f32>, //Temp until Thoth(9/24/2020) is comfortable with how things are setup
    solved_current: Option<f32>, //Temp until Thoth(9/24/2020) is comfortable with how things are setup

    print_voltage: Option<f32>, //Temp until Thoth(9/24/2020) is comfortable with how things are setup
    print_current: Option<f32>, //Temp until Thoth(9/24/2020) is comfortable with how things are setup

    discovered: bool,

    direction : Option<CircuitElementDirection>,

    is_circuit_element: bool, //TODO we will test without using this
    a_index: [Option<usize>; 3],//TODO do we use this?
    b_index: [Option<usize>; 3],//TODO do we use this?

    ac_source_type:  ACSourceType,            //Special
    temp_step_voltage : f32,
    alt_sim_time: f32,

    properties_z: usize,
    label: TinyString,

    bias: f32,
    noise: f32,
    drift: f32,

    initial_altered_rotation: f32,
    time: f32,
}

impl CircuitElement{
    pub const fn empty()->CircuitElement{
        CircuitElement{
            circuit_element_type: SelectedCircuitElement::None,
            orientation: 0.0,
            x: 0,
            y: 0,
            length: 0,
            selected: false,
            selected_rotation: false,

            properties_selected: false,
            properties_move_selected: false,
            properties_offset_x: None,
            properties_offset_y: None,

            resistance: 0.0f32,
            voltage:  0.0f32,
            current:  0.0f32,
            
            unc_resistance: 0.0f32,
            unc_voltage:  0.0f32,
            unc_current:  0.0f32,

            //TODO new (10/26/2020)
            capacitance: 0f32,
            inductance : 0f32,
            charge :     0f32,
            magnetic_flux: 0f32,

            unc_capacitance: 0f32,
            unc_inductance : 0f32,
            unc_charge :     0f32,
            unc_magnetic_flux: 0f32,

            max_voltage : 0f32,
            d_voltage_over_dt: 0f32,
            frequency   : 0f32,

            solved_voltage: None, //Temp until Thoth(9/24/2020) is comfortable with how things are setup
            solved_current: None, //Temp until Thoth(9/24/2020) is comfortable with how things are setup

            print_voltage: None, 
            print_current: None, 

            unique_a_node: 0,
            a_node: 0,
            unique_b_node: 0,
            b_node: 0,

            discovered: false,

            direction : None,

            is_circuit_element: false,
            a_index: [None; 3],
            b_index: [None; 3],

            ac_source_type:  ACSourceType::Sin,
            temp_step_voltage : 0f32,
            alt_sim_time: 0f32,
                
            properties_z: 0,
            label: TinyString::new(),

            bias: 0f32,
            noise: 0f32,
            drift: 0f32,

            initial_altered_rotation: 0f32,
            time: 0f32,
        }

    }
    pub fn new()->CircuitElement{
        let a_id  = get_unique_id();
        let b_id = get_unique_id();


        CircuitElement{
            circuit_element_type: SelectedCircuitElement::None,
            orientation: 0.0,
            x: 0,
            y: 0,
            length: 0,
            selected: false,
            selected_rotation: false,

            properties_selected: false,
            properties_move_selected: false,
            properties_offset_x: None,
            properties_offset_y: None,

            resistance: 0.0f32,
            voltage:  0.0f32,
            current:  0.0f32,

            unc_resistance: 0.0f32,
            unc_voltage:  0.0f32,
            unc_current:  0.0f32,

            capacitance: 0f32,
            inductance : 0f32,
            charge :     0f32,
            magnetic_flux: 0f32,

            unc_capacitance: 0f32,
            unc_inductance : 0f32,
            unc_charge :     0f32,
            unc_magnetic_flux: 0f32,

            max_voltage : 0f32,
            d_voltage_over_dt : 0f32,
            frequency   : 0f32,


            solved_voltage: None, //Temp until Thoth(9/24/2020) is comfortable with how things are setup
            solved_current: None, //Temp until Thoth(9/24/2020) is comfortable with how things are setup

            print_voltage: None, 
            print_current: None, 

            unique_a_node: a_id,
            a_node: a_id,
            unique_b_node: b_id,
            b_node: b_id,

            discovered: false,

            direction : None,

            is_circuit_element: false,
            a_index: [None; 3],
            b_index: [None; 3],

            ac_source_type:  ACSourceType::Sin,
            temp_step_voltage : 0f32,
            alt_sim_time: 0f32,

            properties_z: 0,
            label: TinyString::new(),

            bias: 0f32,
            noise: 0f32,
            drift: 0f32,

            initial_altered_rotation: 0f32,
            time: 0f32,
        }
    }
}

enum SaveLoadEnum{
    Save,
    Load,
    Default,
}

#[derive(PartialEq)]
enum CircuitMenuType{
    Custom,
    Default,
}

#[derive(PartialEq)]
pub enum MessageType{
    Error,
    Default
}

pub struct LS_AppStorage{
    pub init: bool,
    pub eq_storage: EQ_Storage,

    pub menu_canvas: SubCanvas,
    pub menu_move_activated_time: u128,
    pub menu_move_activated: bool,
    pub menu_offscreen: bool,

    pub timer: Instant,
    stop_watch: f32,
    duration: f32, 
    timer_init: bool,

    circuit_element_canvas: SubCanvas,
    selected_circuit_element: SelectedCircuitElement,
    selected_circuit_element_orientation: f32,
    selected_circuit_properties : Option<CircuitElement>,

    resistor_bmp     : TGBitmap,
    battery_bmp      : TGBitmap,
    capacitor_bmp    : TGBitmap,
    inductor_bmp     : TGBitmap,
    voltmeter_bmp    : TGBitmap,
    ammeter_bmp      : TGBitmap,
    switch_open_bmp  : TGBitmap,
    switch_closed_bmp: TGBitmap,
    ac_bmp           : TGBitmap,
    wire_bmp         : TGBitmap,
    custom_bmp       : TGBitmap,


    arrow_bmp              : TGBitmap,
    pub screenshot_icon_bmp: TGBitmap,

    save_icon_bmp    : TGBitmap,
    save_icon_bmp_alt: TGBitmap,

    save_toggle: bool,
    save_toggle_saveload: SaveLoadEnum,

    save_textbox : TextBox,

    arr_circuit_elements: Vec<CircuitElement>,
    arr_panels: Vec<Panel>,
    panel_index: usize,

    saved_circuit_currents: std::collections::HashMap<(usize, usize), [Vec<f32>; 2]>,
    saved_circuit_volts   : std::collections::HashMap<(usize, usize), [Vec<f32>; 2]>,


    teacher_mode: bool,

    lab_text: String,
    panel_previously_modified : std::time::SystemTime,

    run_circuit: bool,

    sim_time: f32,

    circuit_textbox_hash: std::collections::HashMap<(usize, usize), CircuitElementTextBox>,

    messages: Vec<(MessageType, String)>,
    message_index: Option<usize>,
    message_timer: StopWatch,

    custom_circuit_elements: Vec<CircuitElement>,
    circuit_menu_type: CircuitMenuType,
    create_custom_circuit: bool,
    panel_custom_circuit: bool,
    custom_circuit_cursor: usize,
    custom_circuit_textbox: CircuitElementTextBox,

    global_time : StopWatch,
    arrow_toggled : bool,

}



impl LS_AppStorage{
    pub fn  new()->LS_AppStorage{
        LS_AppStorage{
            init: false,
            eq_storage: EQ_Storage::new(),

            menu_canvas: SubCanvas::new(0,0),
            menu_move_activated_time: 0,
            menu_move_activated: false,
            menu_offscreen: false,

            timer: Instant::now(),
            stop_watch: 0f32,//StopWatch::new(),
            duration: 0f32, //Duration::new(0,0),
            timer_init: false,

            circuit_element_canvas: SubCanvas::new(0,0),
            selected_circuit_element: SelectedCircuitElement::None,
            selected_circuit_element_orientation: 0f32,
            selected_circuit_properties : None,

            resistor_bmp: TGBitmap::new(0,0),
            battery_bmp: TGBitmap::new(0,0),
            capacitor_bmp: TGBitmap::new(0,0),
            inductor_bmp : TGBitmap::new(0,0),
            voltmeter_bmp: TGBitmap::new(0,0),
            ammeter_bmp: TGBitmap::new(0,0),
            switch_open_bmp: TGBitmap::new(0,0),
            switch_closed_bmp: TGBitmap::new(0,0),
            ac_bmp: TGBitmap::new(0,0),
            wire_bmp: TGBitmap::new(0,0),
            custom_bmp: TGBitmap::new(0,0),

            arrow_bmp: TGBitmap::new(0,0),
            screenshot_icon_bmp: TGBitmap::new(0,0),
            save_icon_bmp: TGBitmap::new(0,0),
            save_icon_bmp_alt: TGBitmap::new(0,0),

            save_toggle: false,
            save_toggle_saveload: SaveLoadEnum::Default,

            save_textbox : TextBox::new(),

            arr_circuit_elements: Vec::new(),
            arr_panels: Vec::new(),
            panel_index: 0,

            saved_circuit_currents: std::collections::HashMap::new(),
            saved_circuit_volts   : std::collections::HashMap::new(),

            teacher_mode: false,

            lab_text: String::new(),
            panel_previously_modified : std::time::SystemTime::now(),

            run_circuit: false,
            sim_time: 0f32,

            circuit_textbox_hash   : std::collections::HashMap::new(),

            messages: Vec::new(),
            message_index: None,
            message_timer: StopWatch::new(),

            custom_circuit_elements: Vec::new(),
            circuit_menu_type: CircuitMenuType::Default,
            create_custom_circuit: false,
            panel_custom_circuit: false,
            custom_circuit_cursor: 0,
            custom_circuit_textbox: CircuitElementTextBox::new_guess_length(),

            global_time : StopWatch::new(),
            arrow_toggled : false,
        }
    }
}

const example_lab : &str = 
"//
//
#Section
#Text

Hello,
welcome to the circuit simulation software lab. This software is designed for easy and intuitive construction of circuits, as well as their computation. It is an environment where lab TAs and professors can craft lessons for their students. The following panels are an example lab designed to help users acclimate to the software.

Hints:
+ Left click, 2 finger click or double click to access circuit element properties. 
+ Circuit elements can be rotated using property's panel or on grid.
  Hover mouse over far edge of element until you see a rectangle. Click and hold to rotate.
+ Circuit element properties can be changed by typing.
+ Click the download button on the upper right hand of the screen, or top of this panel to save and load circuits.
  When saving circuits be sure to press 'Enter' to complete the save.
+ Current arrows can be toggled using the icon on the top right, behind this panel.
+ A screen shot can be taken using the camera icon on the top right, behind this panel.


#Section
#Header Introduction:
#Text
Circuits are the way with which we make electricity do useful things. And it is through the study of circuits, that we are able to produce many of the tools we use today. In 1827 Ohm published \"The Galvanic Circuit Studied Mathematically\", which experimentally established the relationship between current, I, and potential difference, V. You will verify these relationships.


Voltage, Current and Resistance:
The most simple of circuits contain a battery, the source of the electromotive force, and a resistor, a component through which electricity does work.
The circuit below is one such circuit. Recreate this circuit. Using the panel to the left and place the components on the graph board.
#image circuit.png

#Section
#Text
By pressing \"Run\" the simulation will begin. Double or right click the battery element to access its property. Increase and decrease the voltage, note the circuit's behavior. 
NOTE: Each circuit element can be adjusted using their property panel.

#Question
What occurred when you increased the voltage?
#answerwrong  The speed of the red strips was the same.
#answercorrect The red strips began to move faster.
#answerwrong  The speed slowed down.

#Section
#Text 
The red strips symbolize electrons as they move through the circuit. While noticing visual differences is useful, measurements should be obtained using an ammeter. The ammeter measures current, while the voltmeter measures voltage. Take measurements of the current to determine how voltage and resistance impact the current of a circuit.  Given a data set one can ascertain the relationship between current, voltage and resistance.

#image circuit_ammeter.png

#Question
Construct a simple circuit measure the current for a set of voltages between -2V and 3V.  Using this data set what is the relationship between current and voltage?
#answercorrect linear
#answerwrong  quadratic
#answerwrong  power


#Section
#Text
Scientist often notice patterns in the data they collect then try to construct a mathematical representation of the phenomena encountered. Repeat the measurement from the last slide changing the resistance instead of the voltage. Using the data collected attempt to construct a formula that uses the voltage and resistance to determine the current. Use this to answer the question below. Feel free to check your answers using the simulator.
#Question Assuming a simple resistor, battery circuit what combination of voltage and resistance gives a current of 2.75A?
#answerwrong Voltage: 7V, Resistance: 8Ω 
#answerwrong Voltage: 5V, Resistance: 2Ω
#answercorrect Voltage: 8.25V, Resistance: 3Ω

#Section
#Text
So far you've set the values of circuit components by hand. In the real world the properties of components may not be known, or are known to some finite precision. The 'Custom' tab contains circuit elements with hidden properties.  In this menu you'll find an element labelled 'Resistor_1'.  Use what you learned from previous exercises to determine the resistance of this element.

#Question
What is the resistance of 'Resistor_1'?
#answerwrong 4Ω 
#answercorrect 5Ω 
#answerwrong 2.5Ω 

#Section
#Text
During your studies you may recall your professors and TAs highlighting the impact of uncertainties.  When a resistor, for example, is made the manufacturer will guarantee its value to a certain level of precision. In the 'Custom' tab you will find a circuit element labelled 'R2v1'. Imagine you are constructing an apparatus using this resistor. According to the manufacturer this resistor has 2Ω of resistance. They guarantee the difference in resistance for each resistor to be less than mΩ. Your advisor is skeptical and asks you to measure multiple 'R2v1' resistors to verify their claims.

#Question
Are the 'R2v1' resistors precise to a mΩ?
#answerwrong Yes
#answercorrect No


#Section
#Text
Your advisor contacts the manufacturer and informs them of the issue with 'R2v1'. The manufacturer delivers a new version of the resistor, 'R2v2'.
Measure this new resistors, and take note of their variance and mean.

#Question
Are 'R2v2' resistors perfect 2Ω resistors?
#answerwrong Yes
#answercorrect No



#section
#Header 

END
#text
Thanks for using the lab circuit software. 
#section
#Header APPLICATION DESIGN HISTORY
#text Following contains history of application development.

#Section
#Text
Update(02/09/2021)
+ Fixed sign conventions for voltmeter and ammeter.
+ Changed voltmeter and ammeter icons to include led indicators.
+ Future proof backward save files.
+ Gaussian sampling is now fixed.
+ Implemented fixes for icon and element rotation.
+ Exiting TA mode now requires Tab.
+ Other bug fixes and ui updates.


#Section
#Text
Update(01/26/2021)
+ Changed from uniform to Gaussian uncertainties
+ Implemented screen shot.
+ Added TA password.
+ User generated elements are now destroyed when in overlapping circuit element menu box.
+ Users can now toggle current arrows.
+ Standardizing linux macos and linux side panel visuals.
+ Set frame rate cap.
+ Fixed bugs for circuit compute.
+ Update default side panel.

#Section
#Text
Update(01/08/2021)
+ error banner now implemented
 - errors shown when circuit solution contains NaNs
 - errors shown for TA side panel errors (can not find image, no 'answer' to question, etc.) 
+ Delete key now works to remove text input
+ Property panels now contain a 'z' coordinate value. The last panel interacted with will not be placed in front of others.
+ When interacting with property panels input no longer impact panels or circuit elements obscured by panel
+ For graphs (x, y) information information relative to the mouse locatation was implemeted.
+ Custom, TA defined, circuit elements have been implemented.
+ For custom circuit elements uncertainties can be defined. Uncertainty distribution is currently uniform.
+ small graphics fixes and bugs.


#Section
#Text
Update(12/17/2020)
+Users can now type circuit element parameters
+Window resize now works on all os
+Graphics oddities with window resizing now corrected
+App now longer panics when img is unavailable on startup
+App now resizes to a larger window on startup
+Fixed circuit loop bug **THIS IS BIG** More complicated circuit now work consistently
+MacOS version how has simple executable with icons
+Capacitor animation for negative charges fixed
+AC type now loaded after save
+many small fixes

#Section
#Text
Update(12/04/2020):
+ Graph tweaks
+ duplicate buttons
+ grid rotate element
+ switch and ac elements added
+ current and capacitor animations
+ macos window resize remove
+ changes made to wire rendering

#Section
#Text
Update(11/30/2020):
+ Solver now includes switch and AC elements.
+ Tweaks are made to ui, including graphs.
+ Circuit elements are now be duplicated in an effort to make it easier to add new elements.


#Section
#Text
Update(11/??/2020):
+ solver now includes inductor and capacitor elements.
+ current directions now consistent with battery.
+ solver updates continuously over time.
+ 'TA' mode now included. Use 'Tab' to enter 'TA' mode and 'Tab' to package work.
  + 'TA' mode allows instructors to alter the content of the side panels.
+ To exit teacher mode press the 'Space' key.


#section
#text
Note:
+ Circuit elements must be connected at the edges to be recognized as connected.
Update(10/28/2020):
+ solver now includes voltmeter and ammeters.
+ MacOs version now has functioning 'Teacher Mode'

";


struct PanelFile{
    original_length: u64,
    buffer: Vec<u8>,
}

impl PanelFile{
    fn save(&self, name: &str){
        use std::io::prelude::*;
        
        let mut f = std::fs::File::create(name).expect("File could not be created.");
        f.write_all( b"PF" );
        unsafe{
            f.write_all( &std::mem::transmute::<u64, [u8; 8]>(self.original_length) );
        } 
        f.write( &self.buffer );
    }

    fn load(name: &str)->PanelFile{
        use std::io::prelude::*;
        let mut panelfile = PanelFile{ original_length: 0, buffer: vec![]};
        
        let mut f = std::fs::File::open(name).expect("File could not be created.");
        
        let mut file_header_buffer = [0u8;2];
        f.read( &mut file_header_buffer);
        if file_header_buffer[0] as char == 'P' 
        && file_header_buffer[1] as char == 'F'{
        } else {
            panic!("File type is wrong.");
        }
        
        let mut org_size_buffer = [0u8;8];
        f.read(&mut org_size_buffer);
        unsafe{
            panelfile.original_length = std::mem::transmute(org_size_buffer);
        }

        f.read_to_end( &mut panelfile.buffer);
        return panelfile;
    }
}

fn compress_panelfile( text: &str )->Vec<u8>{
    let compressed = miniz::compress(text.as_bytes());
    return compressed.expect("compress panelfile");
}

fn uncompress_panelfile( panelfile: &PanelFile )->String{
    let len = panelfile.original_length;
    let uncompressed = miniz::uncompress(&panelfile.buffer, len as usize);
    return String::from_utf8_lossy(&uncompressed.expect("uncompressed")).into_owned();
}





pub fn circuit_sim(os_package: &mut OsPackage, app_storage: &mut LS_AppStorage, keyboardinfo: &KeyboardInfo, textinfo: &TextInfo, mouseinfo: &MouseInfo)->i32{

    let window_w = os_package.window_canvas.w;
    let window_h = os_package.window_canvas.h;
    let panel_font = PANEL_FONT;

    if !app_storage.init {

        app_storage.init = true;

        //NOTE we are setting the window size for next frame
        
        if os_package.window_canvas.display_width  != 0 
        && os_package.window_canvas.display_height != 0 {
            if WINDOW_WIDTH >= os_package.window_canvas.display_width {
                os_package.window_info.w = os_package.window_canvas.display_width - 10;
            } else {
                os_package.window_info.w = WINDOW_WIDTH;
            }

            if WINDOW_HEIGHT >= os_package.window_canvas.display_height {
                os_package.window_info.h = os_package.window_canvas.display_height - 10;
            } else {
                os_package.window_info.h = WINDOW_HEIGHT;
            }
        } else {
            os_package.window_info.w = WINDOW_WIDTH;
            os_package.window_info.h = WINDOW_HEIGHT;
        }


        //#["windows"]
        //os_package.window_info.h = 750-70;

        app_storage.menu_canvas = SubCanvas::new( 492, window_h);

        app_storage.circuit_element_canvas = SubCanvas::new(180,350);

        app_storage.save_textbox.text_buffer += "MyCircuit.cd";


        if std::path::Path::new(CIRCUIT_FILE_NAME).is_file(){

            let pf = PanelFile::load(CIRCUIT_FILE_NAME);
            let buffer = miniz::uncompress(&pf.buffer, pf.original_length as usize);
            app_storage.lab_text = String::from_utf8_lossy(&buffer.expect("something went wrong with decompression")).to_string();

        } else {
            app_storage.lab_text = example_lab.to_string();
        }

        let (panels, errors) = parse_and_panel_filebuffer(&app_storage.lab_text);

        app_storage.messages = errors;
        app_storage.arr_panels = panels;

        if std::path::Path::new(CUSTOM_FILE_NAME).is_file(){
            let custom_elements = load_circuit_diagram(CUSTOM_FILE_NAME);
            app_storage.custom_circuit_elements = custom_elements;

        } else {
            app_storage.custom_circuit_elements.clear();
            app_storage.custom_circuit_elements.extend_from_slice(&DEFAULT_CUSTOM_ELEMENTS);
        }


        app_storage.resistor_bmp  = TGBitmap::from_buffer(IMG_RESISTOR);
        app_storage.battery_bmp   = TGBitmap::from_buffer(IMG_BATTERY);
        app_storage.capacitor_bmp = TGBitmap::from_buffer(IMG_CAPACITOR);
        app_storage.inductor_bmp  = TGBitmap::from_buffer(IMG_INDUCTOR);

        app_storage.voltmeter_bmp = TGBitmap::from_buffer(IMG_VOLTMETER);
        app_storage.ammeter_bmp   = TGBitmap::from_buffer(IMG_AMMETER);

        app_storage.switch_open_bmp   = TGBitmap::from_buffer(IMG_SWITCH_OPEN);
        app_storage.switch_closed_bmp = TGBitmap::from_buffer(IMG_SWITCH_CLOSED);
        app_storage.ac_bmp     = TGBitmap::from_buffer(IMG_AC);
        app_storage.wire_bmp   = TGBitmap::from_buffer(IMG_WIRE);
        app_storage.custom_bmp = TGBitmap::from_buffer(IMG_CUSTOM);

        app_storage.arrow_bmp           = TGBitmap::from_buffer(IMG_ARROW);
        app_storage.screenshot_icon_bmp = TGBitmap::from_buffer(IMG_SCREENSHOT);
        app_storage.save_icon_bmp       = TGBitmap::from_buffer(IMG_SAVE);
        app_storage.save_icon_bmp_alt   = TGBitmap::from_buffer(IMG_SAVE_ALT);

        //TODO
        let icon = TGBitmap::from_buffer(IMG_ICON);
        set_icon(&icon);

        change_font(FONT_NOTOSANS);
    }

    //NOTE Frame rate cap of 60 frames per sec.
    {
        match Duration::from_millis(16).checked_sub(app_storage.global_time.get_time()){
            Some(d)=>{
                std::thread::sleep(d);
            },
            _=>{}
        }
        app_storage.global_time.reset()
    }


    let circuit_element_canvas_x_offset = 25;
    let circuit_element_canvas_y_offset = (window_h/2 - app_storage.circuit_element_canvas.canvas.h/2).max(window_h-4*window_h/5);

    //if os_package.window_info.h != app_storage.menu_canvas.canvas.h {
    if window_h != app_storage.menu_canvas.canvas.h {
        app_storage.menu_canvas = SubCanvas::new( 492, window_h);
    }


    //TODO teacher mode turn on
    let mut just_switched_modes = false;
    if keyboardinfo.is_key_released(KeyboardEnum::Tab)
    && std::path::Path::new(TA_FILE_NAME).is_file()
    && app_storage.teacher_mode == false {
        app_storage.teacher_mode = true;
        just_switched_modes = true;

    }

    if app_storage.teacher_mode {
        if keyboardinfo.is_key_released(KeyboardEnum::Tab)
        && !just_switched_modes {
            app_storage.teacher_mode = false;
            app_storage.selected_circuit_element = SelectedCircuitElement::None;
            app_storage.create_custom_circuit = false;
            app_storage.panel_custom_circuit = false;

        }

        let circuit_panel_path = std::path::Path::new("circuit_panels.txt");
        if !circuit_panel_path.exists(){
            let mut f = std::fs::File::create(circuit_panel_path).expect("Could not create file.");
            f.write_all(app_storage.lab_text.as_bytes());

            let date_modified = circuit_panel_path.metadata().expect("Could not retrieve panel file meta data!").modified().unwrap();
            app_storage.panel_previously_modified  = date_modified;

        } else {
            let date_modified = circuit_panel_path.metadata().expect("Could not retrieve panel file meta data!").modified().unwrap();
            if date_modified > app_storage.panel_previously_modified {

                let mut f = std::fs::File::open(circuit_panel_path).expect("Could not open file.");
                app_storage.lab_text.clear();
                f.read_to_string(&mut app_storage.lab_text);
                
                let (mut panels, mut errors) = parse_and_panel_filebuffer(&app_storage.lab_text);

                app_storage.arr_panels.clear();
                app_storage.arr_panels.append( &mut panels );

                app_storage.messages.clear();
                app_storage.messages.append(&mut errors);
                app_storage.message_index = None;

                app_storage.panel_previously_modified = date_modified;

            }
        }

        if keyboardinfo.is_key_pressed(KeyboardEnum::Tab){
            //TODO
            //save slightly encoded using miniz
            let mut panel = PanelFile{ original_length: app_storage.lab_text.as_bytes().len() as u64,
                                   buffer: Vec::new()
                                  };
             

            panel.buffer = compress_panelfile(&app_storage.lab_text);
            panel.save(CIRCUIT_FILE_NAME);//TODO
        }
    }

    if false {
        eq_potential_sim(os_package, app_storage, keyboardinfo, textinfo, mouseinfo);
        return 0;
    }
    


    //Draw bkg color
    draw_rect(&mut os_package.window_canvas, [0, 0, window_w, window_h], COLOR_BKG, true);



    draw_grid(os_package.window_canvas, GRID_SIZE);

    change_font(FONT_MERRIWEATHER_LIGHT);
    let title_length = get_advance_string("Circuit Simulation", 46.0);

    draw_string(&mut os_package.window_canvas, "Circuit Simulation", window_w/2-title_length/2, window_h-60, COLOR_TEXT, 46.0);
    change_font(FONT_NOTOSANS);




    let mut copy_circuit_buffer= vec![];
    for it in app_storage.arr_circuit_elements.iter(){
        copy_circuit_buffer.push(*it);
    } 
    //Change mouse
    match app_storage.selected_circuit_element{
        SelectedCircuitElement::Resistor | SelectedCircuitElement::Battery | SelectedCircuitElement::Capacitor | SelectedCircuitElement::Inductor |
        SelectedCircuitElement::Voltmeter| SelectedCircuitElement::Ammeter | SelectedCircuitElement::Switch | SelectedCircuitElement::AC |
        SelectedCircuitElement::Wire | SelectedCircuitElement::Custom | SelectedCircuitElement::CustomVoltmeter | SelectedCircuitElement::CustomAmmeter=> {


            let x = mouseinfo.x - 50/3;
            let y = mouseinfo.y - 50/3;


            let mut resize_bmp = TGBitmap::new(0,0);

            if app_storage.selected_circuit_element == SelectedCircuitElement::Resistor{
                resize_bmp = sampling_reduction_bmp(&app_storage.resistor_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Battery{
                resize_bmp = sampling_reduction_bmp(&app_storage.battery_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Capacitor{
                resize_bmp = sampling_reduction_bmp(&app_storage.capacitor_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Inductor{
                resize_bmp = sampling_reduction_bmp(&app_storage.inductor_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Voltmeter{
                resize_bmp = sampling_reduction_bmp(&app_storage.voltmeter_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Ammeter{
                resize_bmp = sampling_reduction_bmp(&app_storage.ammeter_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Switch{
                resize_bmp = sampling_reduction_bmp(&app_storage.switch_open_bmp, 80, 80);

            } else if app_storage.selected_circuit_element == SelectedCircuitElement::AC{
                resize_bmp = sampling_reduction_bmp(&app_storage.ac_bmp, 80, 80);
            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Wire{
                resize_bmp = sampling_reduction_bmp(&app_storage.wire_bmp, 80, 80);
            } else if app_storage.selected_circuit_element == SelectedCircuitElement::Custom 
            || app_storage.selected_circuit_element == SelectedCircuitElement::CustomVoltmeter
            || app_storage.selected_circuit_element == SelectedCircuitElement::CustomAmmeter
            {
                resize_bmp = sampling_reduction_bmp(&app_storage.custom_bmp, 80, 80);
            }

            let rotated_bmp = rotate_bmp(&mut resize_bmp, app_storage.selected_circuit_element_orientation, false).unwrap();
            draw_bmp(&mut os_package.window_canvas, &rotated_bmp, x, y, 0.98, None, None);
            if app_storage.selected_circuit_element == SelectedCircuitElement::Custom
            || app_storage.selected_circuit_element == SelectedCircuitElement::CustomVoltmeter
            || app_storage.selected_circuit_element == SelectedCircuitElement::CustomAmmeter{

                let orientation =  app_storage.selected_circuit_element_orientation.sin().abs().round();
                if orientation == 0.0 {
                    let c_it = &app_storage.custom_circuit_elements[app_storage.custom_circuit_cursor];

                    change_font(FONT_NOTOSANS_BOLD);
                    let _adv = get_advance_string(c_it.label.as_ref(), 20f32);
                    draw_string(&mut os_package.window_canvas, c_it.label.as_ref(), x+rotated_bmp.width/2-_adv/2-6, y + 3, C4_WHITE, 20f32);
                    change_font(FONT_NOTOSANS);
                }
            }

            if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down
            && in_rect(mouseinfo.x, mouseinfo.y, [circuit_element_canvas_x_offset, circuit_element_canvas_y_offset, 
                                                    app_storage.circuit_element_canvas.canvas.w, app_storage.circuit_element_canvas.canvas.h]){
                app_storage.selected_circuit_element = SelectedCircuitElement::None;
                app_storage.selected_circuit_element_orientation = 0f32;
                app_storage.selected_circuit_properties = None;
            }

            if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down
            && !in_rect(mouseinfo.x, mouseinfo.y, [circuit_element_canvas_x_offset, circuit_element_canvas_y_offset, 
                                                    app_storage.circuit_element_canvas.canvas.w, app_storage.circuit_element_canvas.canvas.h]){
                let mut element = CircuitElement::new();
                element.circuit_element_type = app_storage.selected_circuit_element; 

                match app_storage.selected_circuit_element{
                    SelectedCircuitElement::Resistor=>{ 
                        match app_storage.selected_circuit_properties{
                            Some(copy_ele) => {
                                element.resistance = copy_ele.resistance;
                            },
                            None=>{
                                element.resistance = DEFAULT_RESISTANCE; 
                            }
                        }
                    },
                    SelectedCircuitElement::Battery=>{ 
                        match app_storage.selected_circuit_properties{
                            Some(copy_ele)=>{
                                element.voltage = copy_ele.voltage; 
                            },
                            None=>{
                                element.voltage = DEFAULT_VOLTAGE; 
                            },
                        }
                    },
                    SelectedCircuitElement::Capacitor=>{ 
                        match app_storage.selected_circuit_properties{
                            Some(copy_ele)=>{
                                element.capacitance = copy_ele.capacitance; 
                            },
                            None=>{
                                element.capacitance = DEFAULT_CAPACITANCE; 
                            },
                        }
                    },
                    SelectedCircuitElement::Inductor=>{ 
                        match app_storage.selected_circuit_properties{
                            Some(copy_ele)=>{
                                element.inductance = copy_ele.inductance; 
                            },
                            None=>{
                                element.inductance = DEFAULT_INDUCTANCE; 
                            },
                        }
                    },
                    SelectedCircuitElement::Voltmeter=>{ 
                        element.resistance = VOLTMETER_RESISTANCE; 
                        app_storage.saved_circuit_volts.insert((element.unique_a_node, element.unique_b_node), [Vec::new(), Vec::new()]);
                    },
                    SelectedCircuitElement::Ammeter=>{ 
                        element.resistance = WIRE_RESISTANCE;
                        app_storage.saved_circuit_currents.insert((element.unique_a_node, element.unique_b_node), [Vec::new(), Vec::new()]);
                    },
                    SelectedCircuitElement::Switch=>{ 
                        match app_storage.selected_circuit_properties{
                            Some(copy_ele)=>{
                                element.resistance = copy_ele.resistance; 
                            },
                            None=>{
                                element.resistance = VOLTMETER_RESISTANCE; 
                            },
                        }
                    },
                    SelectedCircuitElement::AC=>{ 
                        element.voltage = 0.0;
                        element.max_voltage = DEFAULT_VOLTAGE;
                        element.d_voltage_over_dt = DEFAULT_FREQUENCY * 2f32 * PI;
                        element.frequency = DEFAULT_FREQUENCY;
                        element.ac_source_type = ACSourceType::Sin;
                        //TODO maybe
                    },
                    SelectedCircuitElement::Custom |
                    SelectedCircuitElement::CustomVoltmeter |
                    SelectedCircuitElement::CustomAmmeter=>{
                        let c_it = &app_storage.custom_circuit_elements[app_storage.custom_circuit_cursor];

                        if c_it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter { 
                            app_storage.saved_circuit_volts.insert((element.unique_a_node, element.unique_b_node), [Vec::new(), Vec::new()]);
                        }
                        if c_it.circuit_element_type == SelectedCircuitElement::CustomAmmeter { 
                            app_storage.saved_circuit_currents.insert((element.unique_a_node, element.unique_b_node), [Vec::new(), Vec::new()]);
                        }
                        element.circuit_element_type = c_it.circuit_element_type;

                        element.label = c_it.label;

                        element.voltage = c_it.voltage;
                        element.current = c_it.current;
                        element.resistance = c_it.resistance;

                        element.unc_voltage    = sample_normal(c_it.unc_voltage);
                        element.unc_current    = sample_normal(c_it.unc_current);
                        element.unc_resistance = sample_normal(c_it.unc_resistance);

                        element.capacitance = c_it.capacitance;
                        element.inductance  = c_it.inductance;

                        element.unc_capacitance = sample_normal(c_it.unc_capacitance);
                        element.unc_inductance  = sample_normal(c_it.unc_inductance);

                        element.charge = c_it.charge;
                        element.magnetic_flux = c_it.magnetic_flux;

                        element.unc_charge        = sample_normal(c_it.unc_charge);
                        element.unc_magnetic_flux = sample_normal(c_it.unc_magnetic_flux);

                        element.bias  = c_it.bias;
                        element.noise = c_it.noise;
                        element.drift = c_it.drift;
                    },


                    SelectedCircuitElement::Wire=>{
                        element.resistance = WIRE_RESISTANCE;
                    },
                    SelectedCircuitElement::None=>{panic!("Selected circuit should not be a None. Something very wrong occurred.");},
                    _=>{panic!("Selected circuit should not be a WTF it is. Something very wrong occurred.");},
                }


                element.x = (x as f32 / GRID_SIZE as f32).round() as i32 * GRID_SIZE;
                element.y = (y as f32 / GRID_SIZE as f32).round() as i32 * GRID_SIZE;

                element.orientation = app_storage.selected_circuit_element_orientation;

                app_storage.arr_circuit_elements.push(element);
                app_storage.selected_circuit_element = SelectedCircuitElement::None;
                app_storage.selected_circuit_element_orientation = 0f32;
                app_storage.selected_circuit_properties = None;

                app_storage.circuit_textbox_hash.insert((element.unique_a_node, element.unique_b_node), CircuitElementTextBox::new());
            }

        },
        _=>{}
    }

    let mut is_element_selected = false;
    if app_storage.selected_circuit_element != SelectedCircuitElement::None {
        is_element_selected = true;
    }


    {//Draw circuit
     //select circuit element

        fn mouse_in_properties_rect(mouseinfo: &MouseInfo, z_arr: &[(i32, i32, i32, i32)])->bool{
            for it in z_arr.iter(){
                if in_rect(mouseinfo.x, mouseinfo.y, [it.0, it.1, it.2, it.3]){
                    return true;
                }
            }
            return false;
        }

        let mut z_vec = Vec::with_capacity(app_storage.arr_circuit_elements.len());
        for (i_it, it) in app_storage.arr_circuit_elements.iter().enumerate(){
            if it.properties_selected
            && it.properties_offset_x.is_some(){
                let px = it.properties_offset_x.unwrap(); 
                let py = it.properties_offset_y.unwrap();
                let pw = PROPERTIES_W;
                let ph = PROPERTIES_H;
                z_vec.push(( px, py, pw, ph ));
            }
        }

        //NOTE
        //Cicuit Selection loop
        let mut elements_for_removal = vec![];
        for (_i_index, it) in app_storage.arr_circuit_elements.iter_mut().enumerate(){

            if mouseinfo.old_lbutton == ButtonStatus::Up
            && mouseinfo.lbutton == ButtonStatus::Up{
                it.selected = false;
                it.selected_rotation = false;

                let rect1 = [it.x, it.y, 80, 80];
                let rect2 = [circuit_element_canvas_x_offset,
                             circuit_element_canvas_y_offset,
                             app_storage.circuit_element_canvas.canvas.w,
                             app_storage.circuit_element_canvas.canvas.h,
                ];
                
                if overlap_rect_area( rect1, rect2) > 0 {
                    elements_for_removal.push(_i_index);
                }
            }

            if mouseinfo.old_lbutton == ButtonStatus::Up{

                match &it.circuit_element_type{
                    SelectedCircuitElement::Resistor | SelectedCircuitElement::Battery | SelectedCircuitElement::Capacitor | SelectedCircuitElement::Inductor |
                    SelectedCircuitElement::Voltmeter| SelectedCircuitElement::Ammeter | SelectedCircuitElement::Switch | SelectedCircuitElement::AC |
                    SelectedCircuitElement::Wire | SelectedCircuitElement::Custom | SelectedCircuitElement::CustomVoltmeter | SelectedCircuitElement::CustomAmmeter   => {

                      
                        it.x = (it.x as f32 / GRID_SIZE as f32 ).round() as i32 * GRID_SIZE; 
                        it.y = (it.y as f32 / GRID_SIZE as f32 ).round() as i32 * GRID_SIZE;

                        let mut rect = [it.x+2, it.y-4+25, 79+it.length, 40];
                        let mut _mouse_in_rect = [it.x+2, it.y-4+25, 76+it.length, 40];

                        let mut c1_x = it.x + 2 - it.length;
                        let mut c1_y = it.y + 41;

                        let mut c2_x = it.x + it.length + 82;
                        let mut c2_y = it.y + 41;


                        if it.orientation.sin().abs() == 1.0 {
                            rect = [it.x + 12, it.y, 50, 80 + it.length];
                            _mouse_in_rect = [it.x + 12, it.y+4, 40, 73 + it.length];

                            c1_x = it.x + 4 + 38;
                            c1_y = it.y ;

                            c2_x = it.x + 4 + 38;
                            c2_y = it.y + it.length + 3 + 80;
                        }


                        if app_storage.selected_circuit_element == SelectedCircuitElement::None
                        && in_rect( mouseinfo.x, mouseinfo.y, _mouse_in_rect ) 
                        && !mouse_in_properties_rect(mouseinfo, &z_vec){

                            draw_circle(&mut os_package.window_canvas, c1_x, c1_y, 4.0, C4_WHITE); 
                            draw_circle(&mut os_package.window_canvas, c2_x, c2_y, 4.0, C4_WHITE); 

                            //TODO moves should be handled using selected boolean not whether for not mouse is in bounding box of properties if properties are open
                            //or any porperties that have been open
                            if mouseinfo.lbutton == ButtonStatus::Down{
                                it.selected = true;
                            }
                            //TODO should be handled using selected boolean not whether for not mouse is in bounding box of properties if properties are open
                            //or any porperties that have been open
                            if mouseinfo.rclicked()
                            || mouseinfo.double_lbutton{
                                it.properties_selected = true;
                                it.properties_z = get_and_update_global_properties_z();
                                if it.properties_offset_x.is_some() {

                                    let mut properties_x = it.x+it.length+95; 
                                    let mut properties_y = it.y-PROPERTIES_H/2;
                                    it.properties_offset_x = Some(properties_x);
                                    it.properties_offset_y = Some(properties_y);
                                }
                            }
                        }
                        
                        if it.orientation.sin().abs() < 0.001f32{//Horizontal orientation
                            let right_rect = [c2_x-5, c2_y-5, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, right_rect)
                            && !mouse_in_properties_rect(mouseinfo, &z_vec){
                            //Right side

                                draw_circle(&mut os_package.window_canvas, c2_x, c2_y, 4.0, C4_WHITE); 
                                draw_rect(&mut os_package.window_canvas, right_rect, C4_WHITE, false);

                                if mouseinfo.lbutton == ButtonStatus::Down{
                                    it.selected_rotation = true;
                                }
                            }
                            let left_rect = [c1_x-7, c1_y-5, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, left_rect)
                            && !mouse_in_properties_rect(mouseinfo, &z_vec){
                                //Left side

                                draw_circle(&mut os_package.window_canvas, c1_x, c1_y, 4.0, C4_WHITE); 
                                draw_rect(&mut os_package.window_canvas, left_rect, C4_WHITE, false);

                                if mouseinfo.lbutton == ButtonStatus::Down{
                                    it.selected_rotation = true;
                                }
                            }
                        } else if it.orientation.sin().abs() == 1f32{//Vertical
                            let right_rect = [c2_x-6, c2_y-5, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, right_rect)
                            && !mouse_in_properties_rect(mouseinfo, &z_vec){
                                //Top side

                                draw_circle(&mut os_package.window_canvas, c2_x, c2_y, 4.0, C4_WHITE); 
                                draw_rect(&mut os_package.window_canvas, right_rect, C4_WHITE, false);

                                if mouseinfo.lbutton == ButtonStatus::Down{
                                    it.selected_rotation = true;
                                }
                            }
                            let left_rect = [c1_x-6, c1_y-7, 10, 10];//TODO these should be 11x11 to have the result render symmetrically 
                            if in_rect(mouseinfo.x, mouseinfo.y, left_rect)
                            && !mouse_in_properties_rect(mouseinfo, &z_vec){
                                //Bottom side

                                draw_circle(&mut os_package.window_canvas, c1_x, c1_y, 4.0, C4_WHITE); 
                                draw_rect(&mut os_package.window_canvas, left_rect, C4_WHITE, false);

                                if mouseinfo.lbutton == ButtonStatus::Down{
                                    it.selected_rotation = true;
                                }
                            }
                        }
                        if it.selected_rotation {
                            it.initial_altered_rotation = {
                                let d_x = (mouseinfo.x - (it.x + 40)) as f32;//NOTE 40 is half of the bitmap width
                                let d_y = (mouseinfo.y - (it.y + 40)) as f32;//NOTE 40 is half the bitmap height

                                let mut theta  = (d_x/(d_y.powi(2) + d_x.powi(2)).sqrt()).acos();
                                if d_y < 0f32 {
                                    theta *=  -1f32;
                                } 
                                if theta.abs() < 0.1 {
                                    theta += it.orientation;
                                } else {
                                    theta -= it.orientation;
                                }

                                theta
                            };
                        }
                    },
                    _=>{}
               }
            } else {
                match &it.circuit_element_type{
                    SelectedCircuitElement::Resistor | SelectedCircuitElement::Battery | SelectedCircuitElement::Capacitor | SelectedCircuitElement::Inductor |
                    SelectedCircuitElement::Voltmeter| SelectedCircuitElement::Ammeter | SelectedCircuitElement::Switch | SelectedCircuitElement::AC |
                    SelectedCircuitElement::Wire | SelectedCircuitElement::Custom | SelectedCircuitElement::CustomVoltmeter | SelectedCircuitElement::CustomAmmeter=>{
                        if it.selected {
                            is_element_selected = true;
                            it.x += mouseinfo.delta_x;
                            it.y += mouseinfo.delta_y;
                        }
                    },
                    _=>{}
                }
            }

        }
        {//remove circuit elements that are in the circuit elements  
            for it in elements_for_removal.iter().rev(){
                app_storage.arr_circuit_elements.remove(*it);

            }
        }

    }
    {
        //NOTE
        //Render loop
        let mut arrow_resize_bmp = sampling_reduction_bmp(&app_storage.arrow_bmp, 20, 20);

        let mut delete_list = vec![];
        for (i_it, it) in app_storage.arr_circuit_elements.iter_mut().enumerate(){
            match &it.circuit_element_type{
                SelectedCircuitElement::Resistor | SelectedCircuitElement::Battery | SelectedCircuitElement::Capacitor | SelectedCircuitElement::Inductor |
                SelectedCircuitElement::Voltmeter| SelectedCircuitElement::Ammeter | SelectedCircuitElement::Switch | SelectedCircuitElement::AC | 
                SelectedCircuitElement::Wire | SelectedCircuitElement::Custom | SelectedCircuitElement::CustomVoltmeter | SelectedCircuitElement::CustomAmmeter=>{


                    if is_element_selected {

                        let mut c1_x = it.x + 2 - it.length;
                        let mut c1_y = it.y + 41;

                        let mut c2_x = it.x + it.length + 82;
                        let mut c2_y = it.y + 41;

                        if it.orientation.sin().abs() == 1.0 {

                            c1_x = it.x + 4 + 38;
                            c1_y = it.y ;

                            c2_x = it.x + 4 + 38;
                            c2_y = it.y + it.length + 3 + 80;
                        }

                        draw_circle(&mut os_package.window_canvas, c1_x, c1_y, 4.0, C4_WHITE); 
                        draw_circle(&mut os_package.window_canvas, c2_x, c2_y, 4.0, C4_WHITE); 
                    }

                    //TODO implement battery 
                    let node_a = it.unique_a_node;
                    let node_b = it.unique_b_node;

                    let mut temp_bmp = TGBitmap::new(0,0); //TODO ugh this is nasty
                    let mut bmp = &app_storage.resistor_bmp;

                    if it.circuit_element_type == SelectedCircuitElement::Battery{
                        bmp = &app_storage.battery_bmp;
                        if it.voltage < 0.0 {
                            temp_bmp = rotate_bmp(&mut app_storage.battery_bmp, PI, false).unwrap();
                            bmp = &temp_bmp;
                        }
                    } else if it.circuit_element_type == SelectedCircuitElement::Capacitor {
                        bmp = &app_storage.capacitor_bmp;

                    } else if it.circuit_element_type == SelectedCircuitElement::Inductor {
                        bmp = &app_storage.inductor_bmp;
                    } else if it.circuit_element_type == SelectedCircuitElement::Voltmeter {
                        bmp = &app_storage.voltmeter_bmp;
                    } else if it.circuit_element_type == SelectedCircuitElement::Ammeter {
                        bmp = &app_storage.ammeter_bmp;
                    } else if it.circuit_element_type == SelectedCircuitElement::Switch {
                        if it.resistance > 0.0 {
                            bmp = &app_storage.switch_open_bmp;
                        } else {
                            bmp = &app_storage.switch_closed_bmp;
                        }
                    }  else if it.circuit_element_type == SelectedCircuitElement::AC {
                        bmp = &app_storage.ac_bmp;
                    } else if it.circuit_element_type == SelectedCircuitElement::Wire{
                        bmp = &app_storage.wire_bmp;
                    } else if it.circuit_element_type == SelectedCircuitElement::Custom
                    || it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter
                    || it.circuit_element_type == SelectedCircuitElement::CustomAmmeter
                    {
                        bmp = &app_storage.custom_bmp;
                    }



                    let current_good = {  if it.solved_current.is_some() { 
                                              true
                                          }
                                          else { false } 
                                       };
                    let orientation =  it.orientation.sin().abs().round();
                    let mut direction  = 0f32;
                    match it.direction{
                            Some(CircuitElementDirection::AtoB)=>{
                                direction = 1.0;
                            },
                            Some(CircuitElementDirection::BtoA)=>{
                                direction = -1.0;
                            },
                            None=>{},
                    }

                    if it.solved_current.is_none(){
                        it.alt_sim_time = 0f32;
                    }


                    let mut _bmp = sampling_reduction_bmp(bmp, 82, 80);


                    if it.selected_rotation{
                        let d_x = (mouseinfo.x - (it.x + 40)) as f32;//NOTE 40 is half of the bitmap width
                        let d_y = (mouseinfo.y - (it.y + 40)) as f32;//NOTE 40 is half the bitmap height

                        let mut theta  = (d_x/(d_y.powi(2) + d_x.powi(2)).sqrt()).acos() - it.initial_altered_rotation;
                        {
                            if d_y < 0f32 {
                                theta *=  -1f32;
                            } 
                        }


                        let _rbmp = rotate_bmp(&mut _bmp, theta, false).unwrap(); 
                        draw_bmp(&mut os_package.window_canvas, &_rbmp, it.x, it.y, 0.7, None, None);

                        if mouseinfo.lclicked(){
                            //TODO !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                            let _sin = theta.sin().round();
                            let _cos = theta.cos().round();

                            if _sin.abs() <= 0.5 {
                                if _cos >= 0f32 {
                                    theta = 0f32;
                                }
                                else {
                                    theta = PI;
                                }
                            } else { 
                                if _sin >= 0f32 {
                                    theta = PI/2f32;
                                }
                                else { 
                                    theta = 3f32*PI/2f32;
                                }
                            }
                            it.orientation = theta;
                        }
                    }



                    if it.print_current.is_some() 
                    && current_good{
                        let current = *it.print_current.as_ref().unwrap();

                        if app_storage.run_circuit { 
                            it.alt_sim_time += TIME_STEP * current.abs() * direction;
                        }

                        render_current_wire(&mut _bmp, it.alt_sim_time);
                        if it.circuit_element_type == SelectedCircuitElement::Capacitor{//TODO this should be some where else
                            render_charge_capacitor(&mut _bmp, it.charge, it.capacitance);
                        }
                    }

                    if orientation == 0.0 {
                        let mut _bmp = rotate_bmp(&mut _bmp, it.orientation, false).unwrap();
                        draw_bmp(&mut os_package.window_canvas, &_bmp, it.x, it.y, 0.98, None, None);
                        if it.circuit_element_type == SelectedCircuitElement::Custom
                        || it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter
                        || it.circuit_element_type == SelectedCircuitElement::CustomAmmeter{

                            change_font(FONT_NOTOSANS_BOLD);
                            let _adv = get_advance_string(it.label.as_ref(), 20f32);
                            draw_string(&mut os_package.window_canvas, it.label.as_ref(), it.x+_bmp.width/2-_adv/2-6, it.y + 3, C4_WHITE, 20f32);
                            change_font(FONT_NOTOSANS);
                        }
                    }
                    else if orientation == 1.0 {
                        let mut _bmp = rotate_bmp(&mut _bmp, it.orientation, false).unwrap();
                        draw_bmp(&mut os_package.window_canvas, &_bmp, it.x, it.y, 0.98, None, None);

                        if it.circuit_element_type == SelectedCircuitElement::Custom
                        || it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter
                        || it.circuit_element_type == SelectedCircuitElement::CustomAmmeter{

                            change_font(FONT_NOTOSANS_BOLD);
                            let _adv = get_advance_string(it.label.as_ref(), 20f32);
                            draw_string(&mut os_package.window_canvas, it.label.as_ref(), it.x+_bmp.width/2, it.y+it.length/2-5, C4_WHITE, 20f32);
                            change_font(FONT_NOTOSANS);
                        }
                    }

                    let current_good = {  if it.solved_current.is_some() { 
                                              if it.solved_current.as_mut().unwrap().abs() > 0.001 {true } 
                                              else { false }
                                          }
                                          else { false } 
                                       };
                    if current_good 
                    && app_storage.arrow_toggled {
                    //&& app_storage.teacher_mode {
                        match it.direction{
                            Some(CircuitElementDirection::AtoB)=>{
                                if orientation == 0.0 {
                                    let rotated_bmp = rotate_bmp( &mut arrow_resize_bmp, PI, false).unwrap(); //TODO do this some where else further up stream maybe
                                    draw_bmp( &mut os_package.window_canvas, &rotated_bmp, it.x + 30, it.y, 0.98, None, None);
                                } else if orientation == 1.0 {
                                    let rotated_bmp = rotate_bmp(&mut arrow_resize_bmp, PI*3.0/2.0, false).unwrap();
                                    draw_bmp( &mut os_package.window_canvas, &rotated_bmp, it.x+60, it.y +30, 0.98, None, None);
                                }
                            },
                            Some(CircuitElementDirection::BtoA)=>{
                                if orientation == 0.0 {
                                    draw_bmp( &mut os_package.window_canvas, &arrow_resize_bmp, it.x+30, it.y, 0.98, None, None);
                                } else if orientation == 1.0 {
                                    let rotated_bmp = rotate_bmp(&mut arrow_resize_bmp, PI/2.0, false).unwrap();
                                    draw_bmp( &mut os_package.window_canvas, &rotated_bmp, it.x+60, it.y+30, 0.98, None, None);
                                }
                            },
                            None=>{},
                        }
                    }

                },
                _=>{}
            }
        }
        
        let properties_w = PROPERTIES_W; 
        let properties_h = PROPERTIES_H; 

        /////////////
        let mut z_vec = Vec::with_capacity(app_storage.arr_circuit_elements.len());
        for (i_it, it) in app_storage.arr_circuit_elements.iter().enumerate(){
            if it.properties_selected{
                z_vec.push(( i_it, it.properties_z ));
            }
        }
        z_vec.sort_by( |a, b| b.1.cmp(&a.1) );
        for zt in z_vec.iter().rev(){
            let it = &mut app_storage.arr_circuit_elements[zt.0];
            let i_it = zt.0;
            let px = match it.properties_offset_x{
                Some(px) => {px},
                None => { 0 },
            };
            let py = match it.properties_offset_y{
                Some(py) => {py},
                None => { 0 },
            };
            if px != py {
                if in_rect(mouseinfo.x, mouseinfo.y, [px, py, properties_w, properties_h])
                && mouseinfo.old_lbutton == ButtonStatus::Up
                && mouseinfo.lbutton == ButtonStatus::Down
                {
                    if it.properties_z < get_global_properties_z() - 1{
                        it.properties_z = get_and_update_global_properties_z();
                    }
                }
            }
        }
        z_vec.sort_by( |a, b| b.1.cmp(&a.1) );
        for zt in z_vec.iter().rev(){
            let it = &mut app_storage.arr_circuit_elements[zt.0];
            let i_it = zt.0;

            if it.properties_selected {//TODO should this be removed we already know that these elements have properties selected.
                let node_a = it.unique_a_node;
                let node_b = it.unique_b_node;

                let mut label = String::new();
                if it.circuit_element_type != SelectedCircuitElement::Custom
                && it.circuit_element_type != SelectedCircuitElement::CustomVoltmeter
                && it.circuit_element_type != SelectedCircuitElement::CustomAmmeter{
                    label += &format!("{:?}", it.circuit_element_type);
                } else {
                    label += &format!("{}", it.label.as_ref());
                }
                let label = format!("{}: ({} {})", label, node_a, node_b);

                let mut properties_x = it.x+it.length+95; 
                let mut properties_y = it.y-properties_h/2;

                match it.properties_offset_x {
                    None => {
                        it.properties_offset_x = Some(properties_x);
                        it.properties_offset_y = Some(properties_y);
                    },
                    _=>{
                        properties_x = it.properties_offset_x.unwrap();
                        properties_y = it.properties_offset_y.unwrap();
                    }
                }

                if it.properties_z == get_global_properties_z() - 1{
                    draw_rect(&mut os_package.window_canvas, [properties_x, properties_y, properties_w, properties_h], COLOR_PROPERTY_BKG1, true);
                    
                    //NOTE indication that panel can be moved.
                    if in_rect(mouseinfo.x, mouseinfo.y, [properties_x, properties_y+properties_h-30, properties_w, 30]){
                        draw_rect(&mut os_package.window_canvas, [properties_x, properties_y+properties_h-30, properties_w, 30], COLOR_PROPERTY_MOVE2, true);
                    } else {
                        draw_rect(&mut os_package.window_canvas, [properties_x, properties_y+properties_h-30, properties_w, 30], COLOR_PROPERTY_MOVE1, true);
                    }
                } else {
                    draw_rect(&mut os_package.window_canvas, [properties_x, properties_y, properties_w, properties_h], COLOR_PROPERTY_BKG2, true);
                }


                draw_string(&mut os_package.window_canvas, &label, properties_x+2, properties_y+properties_h-30, COLOR_TEXT, 26.0);

                {//Exit properties
                    let char_x_len = get_advance('x', 26.0);
                    let button_x = properties_x + properties_w - get_advance('x', 26.0) - 8;
                    let button_y = properties_y + properties_h - 26;

                    let mut color = C4_WHITE;

                    if in_rect(mouseinfo.x, mouseinfo.y, [button_x, button_y, char_x_len+5, 26]){
                        color = C4_RED;
                        if mouseinfo.lclicked() 
                        && it.properties_z == get_global_properties_z() - 1{
                            it.properties_selected = false;
                        }
                    }
                    draw_string(&mut os_package.window_canvas, "x", button_x, button_y, color, 26.0);
                }


                //Delineation bar between header and info
                draw_rect(&mut os_package.window_canvas, [properties_x+1, properties_y+properties_h-30, properties_w-2, 1], [0.3, 0.3, 0.3, 1.0], true);


                {//move parameter panels
                    if in_rect(mouseinfo.x, mouseinfo.y, [properties_x, properties_y+properties_h-30, properties_w, 30]){

                        if mouseinfo.lbutton == ButtonStatus::Down
                        && it.properties_z == get_global_properties_z() - 1{
                            it.properties_move_selected = true;
                        }
                    }
                    if mouseinfo.lbutton == ButtonStatus::Up{
                        it.properties_move_selected = false;
                    }

                    if it.properties_move_selected {
                        *it.properties_offset_x.as_mut().unwrap() += mouseinfo.delta_x;
                        *it.properties_offset_y.as_mut().unwrap() += mouseinfo.delta_y;
                    }
                }



                {
                    let mut offset_x = 0;
                    let mut offset_y = 55;
                    let textbox = app_storage.circuit_textbox_hash.get_mut(&(it.unique_a_node, it.unique_b_node)).expect("could not find textbox");
                    fn do_text_box_things( tb: &mut TextBox, properties_x: i32, properties_y: i32, properties_w: i32, properties_h: i32, properties_z: usize,
                                           textinfo: &TextInfo, mouseinfo: &MouseInfo, keyboardinfo: &KeyboardInfo,
                                           it_property: &mut f32, window_canvas: &mut WindowCanvas, time: f32,
                                           offset_x : &mut i32, offset_y: i32){

                        tb.x = properties_x+*offset_x;
                        tb.y = properties_y+properties_h-offset_y;

                        let tb_previously_active = tb.active;

                        if properties_z == get_global_properties_z()-1{
                            tb.update(keyboardinfo, textinfo, mouseinfo);
                        }

                        if keyboardinfo.key.contains(&KeyboardEnum::Enter) {
                            match tb.text_buffer.parse::<f32>(){
                                Ok(parsed_f32)=>{ *it_property = parsed_f32; },
                                Err(_)=>{},
                            }
                        }
                        if tb.active{
                            
                        } else {
                            if tb_previously_active {
                                match tb.text_buffer.parse::<f32>(){
                                    Ok(parsed_f32)=>{ *it_property = parsed_f32; },
                                    Err(_)=>{},
                                }
                            }

                            tb.text_buffer.clear();
                            tb.text_cursor = 0;
                            tb.text_buffer += &format!("{:.2}", it_property);
                        }
                        tb.draw(window_canvas, time);
                        *offset_x += tb.max_render_length;
                    }

                    match it.circuit_element_type{
                        SelectedCircuitElement::Wire | 
                        SelectedCircuitElement::Custom=>{ 
                            draw_string(&mut os_package.window_canvas, "No changeable properties.", properties_x, properties_y+properties_h-offset_y, C4_GREY, 20.0);
                            offset_y += 20;

                            if it.circuit_element_type == SelectedCircuitElement::Custom
                            && app_storage.teacher_mode {
                                draw_string(&mut os_package.window_canvas, &format!("Resistance(Ω): {}  {}", it.resistance, it.unc_resistance), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                                draw_string(&mut os_package.window_canvas, &format!("Voltage(V): {}  {}", it.voltage, it.unc_voltage), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                                //draw_string(&mut os_package.window_canvas, &format!("Current(A): {}  {}", it.current, it.unc_current), 
                                //    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                //offset_y += 20;

                                draw_string(&mut os_package.window_canvas, &format!("Capacitance(F): {}  {}", it.capacitance, it.unc_capacitance), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                                draw_string(&mut os_package.window_canvas, &format!("Inductance(H): {}  {}", it.inductance, it.unc_inductance), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                                draw_string(&mut os_package.window_canvas, &format!("Charge(C): {}  {}", it.charge, it.unc_charge), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                                draw_string(&mut os_package.window_canvas, &format!("Flux(Wb): {}  {}", it.magnetic_flux, it.unc_magnetic_flux), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;
                            }
                        },
                        SelectedCircuitElement::Battery=>{ 
                            offset_x += draw_string(&mut os_package.window_canvas, "Voltage(V):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            do_text_box_things(&mut textbox.voltage_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.voltage, &mut  os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);

                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.voltage += 1.0;
                                    //TODO we need to reset solved values it there are solved values
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked() 
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.voltage -= 1.0;
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                             
                        },
                        SelectedCircuitElement::Capacitor=>{ 
                            //TODO need ability to change values
                            offset_x += draw_string(&mut os_package.window_canvas, "Capacitance(F):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);
                            do_text_box_things(&mut textbox.capacitance_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.capacitance, &mut  os_package.window_canvas, 
                                               app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);
                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.capacitance += 0.25;
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);

                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.capacitance -= 0.25;
                                    if it.capacitance < 0f32 { it.capacitance = 0.0; }
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_y += 22;

                            offset_x = 0;
                            offset_x += draw_string(&mut os_package.window_canvas, "Charge(C):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            do_text_box_things(&mut textbox.charge_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.charge, &mut  os_package.window_canvas, 
                                               app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);
                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.charge += 0.25;
                                    if it.charge.is_nan() || it.charge.is_infinite(){
                                        it.charge = 0.0;
                                    }
                                    //TODO we need to reset solved values it there are solved values
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.charge -= 0.25;
                                    if it.charge.is_nan() || it.charge.is_infinite(){
                                        it.charge = 0.0;
                                    }
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);


                        },
                        SelectedCircuitElement::Inductor=>{ 
                            
                            offset_x += draw_string(&mut os_package.window_canvas, "Inductance(H):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);
                            do_text_box_things(&mut textbox.inductance_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.inductance, &mut  os_package.window_canvas, 
                                               app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);
                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.inductance += 0.25;
                                    //TODO we need to reset solved values it there are solved values
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.inductance -= 0.25;
                                    if it.inductance < 0f32 { it.capacitance = 0.0; }
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_y += 22;

                            offset_x = 0;
                            offset_x += draw_string(&mut os_package.window_canvas, "Flux(Wb):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            do_text_box_things(&mut textbox.magflux_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.magnetic_flux, &mut  os_package.window_canvas, 
                                               app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);
                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{

                                    it.magnetic_flux += 1.0;
                                    if it.magnetic_flux.is_nan() || it.magnetic_flux.is_infinite(){
                                        it.magnetic_flux = 0.0;
                                    }
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{

                                    it.magnetic_flux -= 1.0;
                                    if it.magnetic_flux.is_nan() || it.magnetic_flux.is_infinite(){
                                        it.magnetic_flux = 0.0;
                                    }
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);


                        },
                        SelectedCircuitElement::Resistor=>{  
                            offset_x += draw_string(&mut os_package.window_canvas, "Resistance(Ω):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            do_text_box_things(&mut textbox.resistance_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.resistance, &mut  os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);

                            offset_x += 5;
                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.resistance += 1.0;
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.resistance -= 1.0;
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y+1, C4_WHITE, panel_font);
                        },
                        SelectedCircuitElement::Voltmeter | SelectedCircuitElement::CustomVoltmeter=>{  
                            draw_string(&mut os_package.window_canvas, "No changeable properties.", properties_x, properties_y+properties_h-offset_y, C4_GREY, 20.0);
                            if it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter
                            && app_storage.teacher_mode {
                                offset_y += 20;
                                draw_string(&mut os_package.window_canvas, &format!("Bias: {:.3}  Noise: {:.3}  Drift: {:.3}", it.bias, it.noise, it.drift), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                            }

                        },
                        SelectedCircuitElement::Ammeter | SelectedCircuitElement::CustomAmmeter=>{  
                            draw_string(&mut os_package.window_canvas, "No changeable properties.", properties_x, properties_y+properties_h-offset_y, C4_GREY, 20.0);
                            if it.circuit_element_type == SelectedCircuitElement::CustomAmmeter
                            && app_storage.teacher_mode {
                                offset_y += 20;
                                draw_string(&mut os_package.window_canvas, &format!("Bias: : {}  Noise: {}  Drift: {}", it.bias, it.noise, it.drift), 
                                    properties_x, properties_y+properties_h-offset_y, C4_LGREY, 20.0);
                                offset_y += 20;

                            }
                        },
                        SelectedCircuitElement::Switch=>{ 

                            let mut open_color = C4_WHITE; 
                            let mut closed_color = C4_GREY; 
                            if it.resistance == 0.0{
                                open_color = C4_GREY; 
                                closed_color = C4_WHITE; 
                            }


                            let mut _offset = get_advance_string( "Open", 24f32 );
                            let open_rect = [properties_x+5, properties_y+properties_h-offset_y, _offset, 24];

                            if in_rect(mouseinfo.x, mouseinfo.y, open_rect){
                                draw_rect(&mut os_package.window_canvas, open_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.resistance = VOLTMETER_RESISTANCE;
                                }
                            }
                            draw_string(&mut os_package.window_canvas, "Open", properties_x, properties_y+properties_h-offset_y, open_color, 24.0);
                            _offset += get_advance_string("/", 24.0);
                            let __offset = get_advance_string( "Closed", 24.0);

                            let closed_rect = [properties_x+__offset+10, properties_y+properties_h-offset_y, __offset, 24];
                            if in_rect(mouseinfo.x, mouseinfo.y, closed_rect){//CLOSED
                                draw_rect(&mut os_package.window_canvas, closed_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.resistance = 0.0;
                                }
                            }
                            _offset += draw_string(&mut os_package.window_canvas, "/ ", properties_x+_offset, properties_y+properties_h-offset_y, C4_LGREY, 24.0);
                            draw_string(&mut os_package.window_canvas, "Closed", properties_x+_offset, properties_y+properties_h-offset_y, closed_color, 24.0);

                        },
                        SelectedCircuitElement::AC =>{ 
                            
                            offset_x += draw_string(&mut os_package.window_canvas, "Max Voltage(V):   ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            do_text_box_things(&mut textbox.max_voltage_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.max_voltage, &mut  os_package.window_canvas, 
                                               app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);
                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.voltage = it.voltage/it.max_voltage;
                                    it.max_voltage += 1.0;
                                    it.voltage *= it.max_voltage;
                                    //TODO we need to reset solved values it there are solved values
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.voltage = it.voltage/it.max_voltage;
                                    it.max_voltage -= 1.0;
                                    if it.max_voltage < 0f32 { it.max_voltage = 0.0; }
                                    it.voltage *= it.max_voltage;
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_y += 22;

                            offset_x = 0;
                            offset_x += draw_string(&mut os_package.window_canvas, "Frequency(Hz):  ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            do_text_box_things(&mut textbox.frequency_textbox, properties_x, properties_y, properties_w, properties_h, it.properties_z,
                                               textinfo, mouseinfo, keyboardinfo, &mut it.frequency, &mut  os_package.window_canvas, 
                                               app_storage.timer.elapsed().as_secs_f32(),
                                               &mut offset_x, offset_y);
                            offset_x += 5;

                            let plus_rect = [properties_x+offset_x+6, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, plus_rect){
                                draw_rect(&mut os_package.window_canvas, plus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.d_voltage_over_dt = it.d_voltage_over_dt / ( 2f32 * PI * it.frequency );

                                    it.frequency += 0.25;

                                    it.d_voltage_over_dt = it.d_voltage_over_dt * it.frequency * 2f32 * PI;

                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '+', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);
                            offset_x += 10;

                            let minus_rect = [properties_x+offset_x+4, properties_y+properties_h-offset_y+9, 10, 10];
                            if in_rect(mouseinfo.x, mouseinfo.y, minus_rect){
                                draw_rect(&mut os_package.window_canvas, minus_rect, C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.d_voltage_over_dt = it.d_voltage_over_dt / ( 2f32 * PI * it.frequency );

                                    let _angle = it.d_voltage_over_dt.acos() / it.frequency;

                                    it.frequency -= 0.25;
                                    if it.frequency < 0f32 {
                                        it.frequency = 0.00001;
                                    }
                                    it.d_voltage_over_dt = it.d_voltage_over_dt * it.frequency * 2f32 * PI;
                                }
                            }
                            offset_x += draw_char(&mut os_package.window_canvas, '-', properties_x+2+offset_x, properties_y+properties_h-offset_y, C4_WHITE, panel_font);

                            offset_y += 22;
                            offset_x = 0;
                            offset_x += draw_string(&mut os_package.window_canvas, "Source Type: ", properties_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            let mut step_color = C4_GREY;
                            let mut sin_color = C4_LGREY;
                            if it.ac_source_type == ACSourceType::Step{
                                step_color = C4_WHITE;
                                sin_color = C4_GREY;
                            } 

                            let sin_offset_x = get_advance_string("Sin ", panel_font);
                            if in_rect(mouseinfo.x, mouseinfo.y, [properties_x+offset_x+4, properties_y+properties_h-offset_y, sin_offset_x, panel_font as i32]){
                                draw_rect(&mut os_package.window_canvas, [properties_x+offset_x+4, properties_y+properties_h-offset_y, sin_offset_x, panel_font as i32], C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.ac_source_type = ACSourceType::Sin;
                                }
                            }
                            draw_string(&mut os_package.window_canvas, "Sin ", properties_x+offset_x+2, properties_y+properties_h-offset_y, sin_color, panel_font);
                            offset_x += sin_offset_x;

                            offset_x += draw_string(&mut os_package.window_canvas, "/ ", properties_x+offset_x+2, properties_y+properties_h-offset_y, C4_GREY, panel_font);

                            let step_offset_x = get_advance_string( "Step ", panel_font);
                            if in_rect(mouseinfo.x, mouseinfo.y, [properties_x+offset_x+4, properties_y+properties_h-offset_y, step_offset_x, panel_font as i32]){
                                draw_rect(&mut os_package.window_canvas, [properties_x+offset_x+4, properties_y+properties_h-offset_y, step_offset_x, panel_font as i32], C4_DGREY, true);
                                if mouseinfo.lclicked()
                                && it.properties_z == get_global_properties_z() - 1{
                                    it.ac_source_type = ACSourceType::Step;
                                }
                            }
                            draw_string(&mut os_package.window_canvas, "Step ", properties_x+offset_x+2, properties_y+properties_h-offset_y, step_color, panel_font);
                            offset_x += step_offset_x;

                        },
                        _=>{panic!("TODO");}
                    }

                    offset_y += 22;
                    if app_storage.teacher_mode {
                        match it.print_current{
                            Some(c)=>{ 
                                draw_string(&mut os_package.window_canvas, &format!("Solv. Current(A): {:.2}", c), properties_x+2, properties_y+properties_h-offset_y, COLOR_TEXT_SOLV, panel_font);
                            },
                            None=>{},
                        } 

                        offset_y += 20;
                        match it.print_voltage{
                            Some(v)=>{
                                draw_string(&mut os_package.window_canvas, &format!("Voltage Diff(V): {:.2}", v), properties_x+2, properties_y+properties_h-offset_y, COLOR_TEXT_SOLV, panel_font);
                            },
                            None=>{},
                        } 
                    } else {
                        if it.circuit_element_type == SelectedCircuitElement::Voltmeter || it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter{
                            match it.print_voltage{
                                Some(v)=>{
                                    let voltage = v;
                                    draw_string(&mut os_package.window_canvas, &format!("Voltage (V): {:.2}", voltage), properties_x+2, properties_y+properties_h-offset_y, COLOR_TEXT_SOLV, panel_font);

                                    offset_y += (properties_h/2 - 5).max(45);
                                    let y_x = app_storage.saved_circuit_volts.get_mut(&(it.unique_a_node, it.unique_b_node)).expect("Could not find node");
                                    draw_graph(&mut os_package.window_canvas, &y_x[1], &y_x[0], 
                                               [properties_x+2, properties_y+properties_h-offset_y, properties_w-4, (properties_h/2).max(50)], 
                                               5.0, 18.0, mouseinfo);

                                    offset_y += 22;
                                    let save_rect = [properties_x+properties_w-62, properties_y+properties_h-offset_y, 60, 20];
                                    let mut save_rect_color = C4_DGREY;

                                    if in_rect(mouseinfo.x, mouseinfo.y, save_rect){
                                        save_rect_color = C4_GREY;
                                        if mouseinfo.lclicked()
                                        && it.properties_z == get_global_properties_z() - 1{
                                            save_csv(&format!("voltmeter_{}_{}.csv", node_a, node_b), &[&y_x[1], &y_x[0]], &["time(s)", "voltage(V)"]);
                                            app_storage.messages.push( (MessageType::Default, format!("Message: voltmeter_{}_{}.csv saved.", node_a, node_b)) );
                                        }
                                    }
                                    draw_rect(&mut os_package.window_canvas, save_rect, save_rect_color, true);
                                    draw_string(&mut os_package.window_canvas, "Save", save_rect[0]+8, save_rect[1]-4, C4_WHITE, 22.0);


                                    let clear_rect = [properties_x+properties_w-124, properties_y+properties_h-offset_y, 60, 20];
                                    let mut clear_rect_color = C4_DGREY;

                                    if in_rect(mouseinfo.x, mouseinfo.y, clear_rect){
                                        clear_rect_color = C4_GREY;
                                        if mouseinfo.lclicked()
                                        && it.properties_z == get_global_properties_z() - 1{
                                            y_x[0].clear();
                                            y_x[1].clear();
                                            it.time = 0f32;
                                        }
                                    }
                                    draw_rect(&mut os_package.window_canvas, clear_rect, clear_rect_color, true);
                                    draw_string(&mut os_package.window_canvas, "Clear", clear_rect[0]+8, clear_rect[1]-4, C4_WHITE, 22.0);
                                },
                                None=>{},
                            } 
                        } 
                        if it.circuit_element_type == SelectedCircuitElement::Ammeter || it.circuit_element_type == SelectedCircuitElement::CustomAmmeter{
                            match it.print_current{
                                Some(c)=>{ 
                                    let current = c;
                                    draw_string(&mut os_package.window_canvas, &format!("Current(A): {:.2}", current), properties_x+2, properties_y+properties_h-offset_y, COLOR_TEXT_SOLV, panel_font);

                                    offset_y += (properties_h/2 - 5).max(45);
                                    let y_x = app_storage.saved_circuit_currents.get_mut(&(it.unique_a_node, it.unique_b_node)).expect("Could not find node");
                                    draw_graph(&mut os_package.window_canvas, &y_x[1], &y_x[0], 
                                               [properties_x+2, properties_y+properties_h-offset_y, properties_w-4, (properties_h/2).max(50)], 
                                               5.0, 18.0, mouseinfo);

                                    offset_y += 22;
                                    let save_rect = [properties_x+properties_w-62, properties_y+properties_h-offset_y, 60, 20];
                                    let mut save_rect_color = C4_DGREY;

                                    if in_rect(mouseinfo.x, mouseinfo.y, save_rect){
                                        save_rect_color = C4_GREY;
                                        if mouseinfo.lclicked()
                                        && it.properties_z == get_global_properties_z() - 1{
                                            save_csv(&format!("ammeter_{}_{}.csv", node_a, node_b), &[&y_x[1], &y_x[0]], &["time(s)", "current(A)"]);
                                            app_storage.messages.push( (MessageType::Default, format!("Message: ammeter_{}_{}.csv saved.", node_a, node_b)) );
                                        }
                                    }
                                    draw_rect(&mut os_package.window_canvas, save_rect, save_rect_color, true);
                                    draw_string(&mut os_package.window_canvas, "Save", save_rect[0]+8, save_rect[1]-4, C4_WHITE, 22.0);

                                    let clear_rect = [properties_x+properties_w-124, properties_y+properties_h-offset_y, 60, 20];
                                    let mut clear_rect_color = C4_DGREY;

                                    if in_rect(mouseinfo.x, mouseinfo.y, clear_rect){
                                        clear_rect_color = C4_GREY;
                                        if mouseinfo.lclicked()
                                        && it.properties_z == get_global_properties_z() - 1{
                                            y_x[0].clear();
                                            y_x[1].clear();
                                            it.time = 0f32;
                                        }
                                    }
                                    draw_rect(&mut os_package.window_canvas, clear_rect, clear_rect_color, true);
                                    draw_string(&mut os_package.window_canvas, "Clear", clear_rect[0]+8, clear_rect[1]-4, C4_WHITE, 22.0);

                                },
                                None=>{},
                            } 
                        }
                    }
                }

                //Delineation bar between info and image manipulation
                draw_rect(&mut os_package.window_canvas, [properties_x+1, properties_y+35, properties_w-2, 1], C4_DGREY, true);


                //TODO I should implement this
                //fn do_button(canvas: WindowCanvas, string: &str, x: i32, y: i32, mouseinfo: &mouseinfo, extra: Option<ButtonExtras>)->VButtonStatus{down: bool, clicked: bool, over: bool, button_width};
                //struct ButtonExtras{ x_pad, y_pad, text_color, text_alt_color, bkg_color, bkg_alt_color, font_size }
                ////Rotate Button
                let rotation_button_width = {
                    let button_x = properties_x+5;
                    let button_w = get_advance_string("Rotate", 22.0) + 10; //TODO something weird is going on with get_advanced. something I don't understand. the width is not correct
                    let button_h = 25;
                    let button_y = properties_y + 5;

                    draw_rect(&mut os_package.window_canvas, [button_x, button_y, button_w, button_h], C4_DGREY, true);
                    if in_rect(mouseinfo.x, mouseinfo.y,[button_x, button_y, button_w, button_h]){
                        draw_rect(&mut os_package.window_canvas, [button_x, button_y, button_w, button_h], C4_GREY, true);
                        if mouseinfo.lclicked()
                        && it.properties_z == get_global_properties_z() - 1{
                            it.orientation += PI/2.0;

                            //if (it.orientation/PI).abs().fract() < 0.001 { it.orientation = 0.0; }
                            //else if (it.orientation/(PI/2.0)).abs().fract() < 0.001 { it.orientation = PI/2.0; }
                        }
                    }
                    draw_string(&mut os_package.window_canvas, "Rotate", button_x, button_y-3, C4_LGREY, 22.0);
                    button_w
                };

                
                ////Delete Button
                let delete_button_width = {
                    let button_x = properties_x+rotation_button_width+10;
                    let button_w = get_advance_string("Delete", 22.0) + 10; //TODO something weird is going on with get_advanced. something I don't understand. the width is not correct
                    let button_h = 25;
                    let button_y = properties_y + 5;

                    draw_rect(&mut os_package.window_canvas, [button_x, button_y, button_w, button_h], C4_DGREY, true);
                    if in_rect(mouseinfo.x, mouseinfo.y,[button_x, button_y, button_w, button_h]){
                        draw_rect(&mut os_package.window_canvas, [button_x, button_y, button_w, button_h], C4_GREY, true);
                        if mouseinfo.lclicked() 
                        && it.properties_z == get_global_properties_z() - 1{
                            delete_list.push(i_it);
                        }
                    }
                    draw_string(&mut os_package.window_canvas, "Delete", button_x, button_y-3, C4_LGREY, 22.0);
                    button_w
                };
                ////Duplicate Button
                {
                    let button_x = properties_x+rotation_button_width+delete_button_width+15;
                    let button_w = get_advance_string("Delete", 22.0) + 10; //TODO something weird is going on with get_advanced. something I don't understand. the width is not correct
                    let copy_w = get_advance_string("Copy", 22.0) + 10; //TODO something weird is going on with get_advanced. something I don't understand. the width is not correct
                    let button_h = 25;
                    let button_y = properties_y + 5;

                    draw_rect(&mut os_package.window_canvas, [button_x, button_y, button_w, button_h], C4_DGREY, true);
                    if in_rect(mouseinfo.x, mouseinfo.y,[button_x, button_y, button_w, button_h]){
                        draw_rect(&mut os_package.window_canvas, [button_x, button_y, button_w, button_h], C4_GREY, true);

                        if mouseinfo.lclicked()
                        && it.properties_z == get_global_properties_z() - 1{
                            app_storage.selected_circuit_element = it.circuit_element_type;
                            app_storage.selected_circuit_element_orientation = it.orientation;
                            it.properties_selected = false;

                            let mut copied_circuit = CircuitElement::new();
                            copied_circuit.resistance     = it.resistance;
                            copied_circuit.voltage        = it.voltage;
                            copied_circuit.current        = it.current;
                            copied_circuit.capacitance    = it.capacitance;
                            copied_circuit.inductance     = it.inductance;
                            copied_circuit.charge         = it.charge;
                            copied_circuit.magnetic_flux  = it.magnetic_flux;
                            copied_circuit.max_voltage    = it.max_voltage;
                            copied_circuit.d_voltage_over_dt  = it.d_voltage_over_dt;
                            copied_circuit.frequency       = it.frequency;
                            copied_circuit.ac_source_type  = it.ac_source_type;

                            copied_circuit.bias   = it.bias;
                            copied_circuit.noise  = it.noise;
                            copied_circuit.drift = it.drift;

                            app_storage.selected_circuit_properties = Some(copied_circuit);
                        }
                    }
                    draw_string(&mut os_package.window_canvas, "Copy", button_x + (button_w - copy_w)/2, button_y-3, C4_LGREY, 22.0);
                }


            }


        }
        ////////////////////////////

        loop{
            let it = delete_list.pop();
            match it {
                Some(n)=>{ 
                    let a_b = (app_storage.arr_circuit_elements[n].unique_a_node, app_storage.arr_circuit_elements[n].unique_b_node);
                    app_storage.saved_circuit_volts.remove(&a_b);
                    app_storage.saved_circuit_currents.remove(&a_b);
                    app_storage.arr_circuit_elements.remove(n); 
                },
                None=>{break;},
            }
        }
    }




    //////////////////////////////////////////////////////////////////////
    let mut string_y = 0;
    let temp_w = app_storage.menu_canvas.canvas.w;
    let temp_h = app_storage.menu_canvas.canvas.h;
    //Draw Background  Color
    draw_rect(&mut app_storage.menu_canvas.canvas, [0, 0, temp_w, temp_h], COLOR_MENU_BKG, true);

    {//Render panels
        let i = app_storage.panel_index;
        let panel = &mut app_storage.arr_panels[i];
        let mut current_cursor_position = temp_h - 35;

        for p in panel.contents.iter_mut(){
            match p {
                PanelContent::Image(im) => {
                    let ratio_w =  im.width as f32 / temp_w  as f32 ;
                    let mut w = None;
                    let mut h = None;
                    if ratio_w > 0.6{
                        w = Some( (0.6 * temp_w as f32) as i32 ); 
                        h = Some(((0.6 * temp_w as f32 / im.width as f32) * im.height as f32) as i32 );  
                        current_cursor_position -= ((0.6 * temp_w as f32 / im.width as f32) * im.height as f32) as i32;
                    } else {
                        current_cursor_position -= im.height;
                    }

                    let x = match w { Some(_w)=>{ temp_w/2 - _w/2  }, None=>{ temp_w/2 - im.width/2}  } ;
                    draw_bmp(&mut app_storage.menu_canvas.canvas, im, x, current_cursor_position, 0.999, w, h);
                },
                PanelContent::Text(text) => {
                    let strings = generate_wrapped_strings(&text, 26.0, temp_w);
                    //TODO
                    //cap length.
                    for i in 0..strings.len(){
                        current_cursor_position -= 24;
                        draw_string(&mut app_storage.menu_canvas.canvas, &strings[i], 10, current_cursor_position, COLOR_TEXT, 26.0);
                    }
                },
                PanelContent::Header(text) => {
                    change_font(FONT_NOTOSANS_BOLD);
                    let strings = generate_wrapped_strings(&text, 32.0, temp_w);
                    for i in 0..strings.len(){
                        current_cursor_position -= 30;
                        draw_string(&mut app_storage.menu_canvas.canvas, &strings[i], 10, current_cursor_position, COLOR_TEXT, 32.0);
                    }
                    change_font(FONT_NOTOSANS);
                },
                PanelContent::Question(mq) => {
                    if mq.answer_index.is_none() { 
                        //TODO we have a problem
                        panic!();
                    }

                    change_font(FONT_MERRIWEATHER_LIGHT);
                    let strings = generate_wrapped_strings(&mq.question, 26.0, temp_w);
                    change_font(FONT_NOTOSANS);

                    current_cursor_position -=  26;

                    change_font(FONT_NOTOSANS_BOLD);
                    draw_string(&mut app_storage.menu_canvas.canvas, "Question:", 10, current_cursor_position, COLOR_TEXT, 30.0);
                    change_font(FONT_NOTOSANS);

                    current_cursor_position -=  30;
                    for i in 0..strings.len(){
                    change_font(FONT_MERRIWEATHER_LIGHT);
                        draw_string(&mut app_storage.menu_canvas.canvas, &strings[i], 10, current_cursor_position, COLOR_TEXT, 24.0);
                    change_font(FONT_NOTOSANS);
                        current_cursor_position -=  24;
                    }


                    current_cursor_position +=  20;
                    let choice_arr = vec!["A)", "B)","C)","D)","E)", "F)", "H)"];
                    let mut rects_arr = vec![]; 
                    for i in 0..mq.choices.len() {
                        let strings = generate_wrapped_strings(&mq.choices[i], 24.0, temp_w);
                        let string_width = draw_string(&mut app_storage.menu_canvas.canvas, &choice_arr[i], 3, current_cursor_position, COLOR_TEXT, 24.0);
                        rects_arr.push([3, current_cursor_position+2, string_width + 3, 24]); 

                        for j in 0..strings.len(){
                            rects_arr[i][2] += draw_string(&mut app_storage.menu_canvas.canvas, &strings[j], 20, current_cursor_position, COLOR_TEXT, 24.0);
                            current_cursor_position -= 13;
                        }
                    }

                    if mq.answers.len() < mq.number_chances{ //TODO how should this work... I don't know
                        for i in 0..rects_arr.len(){

                            //TODO make this general
                            if in_rect(mouseinfo.x - window_w + temp_w, mouseinfo.y, rects_arr[i]){
                                draw_rect(&mut app_storage.menu_canvas.canvas, rects_arr[i], C4_YELLOW, false);

                                if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down{
                                    mq.answers.push(i);
                                    if i == mq.answer_index.unwrap() { mq.number_chances = 0; } //NOTE: we may not want to use number of chances in this way. It changes the meaning of the phrase.
                                }
                            }
                        }
                        if mq.answers.len() > 0 {
                            let index = mq.answers.len()-1;
                            if mq.answers[index] != mq.answer_index.unwrap() {
                                change_font(FONT_NOTOSANS_BOLD);
                                draw_string(&mut app_storage.menu_canvas.canvas, &format!("Incorrect: {}/{}", mq.answers.len(), mq.number_chances), 2, current_cursor_position+15, C4_DGREY, 22.0);
                                change_font(FONT_NOTOSANS);
                            }
                        }
                    } else {
                        if mq.answers.len() > 0 {
                            let index = mq.answers.len()-1;
                            if mq.answers[index] == mq.answer_index.unwrap() {
                                change_font(FONT_NOTOSANS_BOLD);
                                draw_string(&mut app_storage.menu_canvas.canvas, "Correct", 2, current_cursor_position+15, C4_DGREY, 22.0);
                                change_font(FONT_NOTOSANS);
                            } else {
                                change_font(FONT_NOTOSANS_BOLD);
                                draw_string(&mut app_storage.menu_canvas.canvas, &format!("Incorrect: The answer is ({}", choice_arr[mq.answer_index.unwrap()]), 2, current_cursor_position+15, C4_DGREY, 22.0);
                                change_font(FONT_NOTOSANS);
                            }
                        }
                    }


                },
                _=>{},
            }
        }
        if i < app_storage.arr_panels.len()-1
        && !app_storage.menu_offscreen{
            let x = os_package.window_canvas.w - app_storage.menu_canvas.canvas.w;

            if in_rect(mouseinfo.x - x, mouseinfo.y, [temp_w-102, 10, 102, 30]){
                draw_rect(&mut app_storage.menu_canvas.canvas, [temp_w-102, 10, 102, 30], C4_GREY, true);
                if mouseinfo.lclicked() {
                    app_storage.panel_index += 1;
                }
            } else {
                draw_rect(&mut app_storage.menu_canvas.canvas, [temp_w-102, 10, 102, 30], C4_DGREY, true);
            } 
            draw_string(&mut app_storage.menu_canvas.canvas, "Continue", temp_w - 100, 10, C4_WHITE, 26.0);
        } 
        if i > 0 
        && !app_storage.menu_offscreen{
            let x = os_package.window_canvas.w - app_storage.menu_canvas.canvas.w;

            if in_rect(mouseinfo.x - x, mouseinfo.y, [0, 10, 102, 30]){
                draw_rect(&mut app_storage.menu_canvas.canvas, [0, 10, 102, 30], C4_GREY, true);
                if mouseinfo.lclicked() {
                    app_storage.panel_index -= 1;
                }
            } else {
                draw_rect(&mut app_storage.menu_canvas.canvas, [0, 10, 102, 30], C4_DGREY, true);
            } 
            draw_string(&mut app_storage.menu_canvas.canvas, "Previous", 2, 10, C4_WHITE, 26.0);
        }
    }


    {//NOTE toggle arrows
        let arrow_rect = [ window_w - 125, window_h - 25, 20, 20];
        let mut arrow_bmp = sampling_reduction_bmp(&app_storage.arrow_bmp, 22,22);
        
        if in_rect(mouseinfo.x, mouseinfo.y, arrow_rect) {
            if app_storage.arrow_toggled { render_red(&mut arrow_bmp)}
            else {render_grey(&mut arrow_bmp);}

            if mouseinfo.lclicked()
            && app_storage.menu_offscreen {
                app_storage.arrow_toggled = !app_storage.arrow_toggled;
            }
        } else {
            if app_storage.arrow_toggled { render_pink(&mut arrow_bmp) }
        }
        draw_bmp(&mut os_package.window_canvas, &arrow_bmp, arrow_rect[0], arrow_rect[1], 0.98, None, None);
    }

    {//NOTE screen shot
 
        let screenshot_rect = [ window_w - 95, window_h - 25, 20, 20];

        let mut icon_bmp = app_storage.screenshot_icon_bmp.clone(); 
        
        if in_rect(mouseinfo.x, mouseinfo.y, screenshot_rect) {
            render_grey(&mut icon_bmp);

            if mouseinfo.lclicked()
            && app_storage.menu_offscreen {
                let mut bmp = TGBitmap::new(os_package.window_canvas.w, os_package.window_canvas.h);


                bmp.file_header.off_bits = 54; //TODO
                bmp.file_header.size_ = (4*bmp.width * bmp.height) as u32 + bmp.file_header.off_bits;
                bmp.info_header.header_size = 40;
                bmp.info_header.compression = 0;
                bmp.info_header.image_size = (4*bmp.width * bmp.height) as u32;
                bmp.info_header.x_px_per_meter = 1;
                bmp.info_header.y_px_per_meter = 1;

                unsafe{
                    let buffer = os_package.window_canvas.buffer as *const u8;
                    for i in 0..(bmp.width*bmp.height) as usize{
                        bmp.rgba[4*i + 0] = *buffer.offset(4*i  as isize + 0);
                        bmp.rgba[4*i + 1] = *buffer.offset(4*i  as isize + 1);
                        bmp.rgba[4*i + 2] = *buffer.offset(4*i  as isize + 2);
                        bmp.rgba[4*i + 3] = *buffer.offset(4*i  as isize + 3);
                    }
                }

                bmp.save_bmp("screenshot.bmp");//TODO search for conflicts
                app_storage.messages.push((MessageType::Default, "Message: screenshot saved to screenshot.bmp".to_string()));
            }
        }
        draw_bmp(&mut os_package.window_canvas, &icon_bmp, screenshot_rect[0], screenshot_rect[1], 0.98, Some(20), Some(20));
    }





    
    //////////////////////////////////////////////////////////////////////
    //NOTE draw circuit ingredient panel
    {
        let subcanvas_w = app_storage.circuit_element_canvas.canvas.w;
        let subcanvas_h = app_storage.circuit_element_canvas.canvas.h;

        let x_offset = circuit_element_canvas_x_offset;
        let y_offset = circuit_element_canvas_y_offset;
        
        let mut standard_color = C4_WHITE;
        let mut unique_color = C4_DGREY;
        if app_storage.circuit_menu_type != CircuitMenuType::Default{
            standard_color = C4_DGREY;
            unique_color = C4_WHITE;
        }

        {//Standard button
            let std_rect = [25, y_offset+subcanvas_h-1, subcanvas_w/2, 25];
             
            draw_rect(&mut os_package.window_canvas, std_rect, standard_color, true);
            if in_rect(mouseinfo.x, mouseinfo.y, std_rect)
            && mouseinfo.lclicked(){
                app_storage.circuit_menu_type = CircuitMenuType::Default;
            }

            change_font(FONT_NOTOSANS_BOLD);
            let adv = get_advance_string("Standard", 24.0) + 4;
            draw_string(&mut os_package.window_canvas, "Standard", std_rect[0]+subcanvas_w/4-adv/2, std_rect[1]-4, C4_GREY, 24.0);
            change_font(FONT_NOTOSANS);
        } 

        {//Custom button
            let unq_rect = [25 + subcanvas_w/2, y_offset+subcanvas_h-1, subcanvas_w/2-1, 25];
            draw_rect(&mut os_package.window_canvas, unq_rect, unique_color, true);
            if in_rect(mouseinfo.x, mouseinfo.y, unq_rect)
            && mouseinfo.lclicked(){
                app_storage.circuit_menu_type = CircuitMenuType::Custom;
            }

            change_font(FONT_NOTOSANS_BOLD);
            let adv = get_advance_string("Custom", 24.0) + 4;
            draw_string(&mut os_package.window_canvas, "Custom", 25+(3*subcanvas_w)/4-adv/2, unq_rect[1]-4, C4_GREY, 24.0);
            change_font(FONT_NOTOSANS);
        } 

        if app_storage.teacher_mode 
        && app_storage.circuit_menu_type == CircuitMenuType::Custom{//Save button
            let save_rect = [25 + subcanvas_w/2, y_offset-25, subcanvas_w/2-1, 25];
            draw_rect(&mut os_package.window_canvas, save_rect, C4_RED, true);
            if in_rect(mouseinfo.x, mouseinfo.y, save_rect)
            && mouseinfo.lclicked(){
                save_circuit_diagram(CUSTOM_FILE_NAME, &app_storage.custom_circuit_elements);
                app_storage.messages.push((MessageType::Default, "Message: A custom circuit set has been saved.".to_string()));
            }

            change_font(FONT_NOTOSANS_BOLD);
            let adv = get_advance_string("Save", 24.0) + 4;
            draw_string(&mut os_package.window_canvas, "Save", save_rect[0] + save_rect[2]/2 - adv/2, save_rect[1]-4, C4_WHITE, 24.0);
            change_font(FONT_NOTOSANS);
        }

        if app_storage.circuit_menu_type == CircuitMenuType::Custom{
            draw_rect(&mut app_storage.circuit_element_canvas.canvas, [0, 0, subcanvas_w, subcanvas_h], C4_WHITE, true);
            draw_rect(&mut app_storage.circuit_element_canvas.canvas, [3, 3, subcanvas_w-6, subcanvas_h-6], COLOR_MENU_BKG, true);
            
            let mut index : i32 = 0;
            for it in app_storage.custom_circuit_elements.iter(){
                let rect = [80*(index%2)+5+4, subcanvas_h - 70*((index+2)/2) - 5, 80, 70];
                let _rect = [rect[0]-15+x_offset, rect[1]-15 + y_offset, 80, 70];


                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.custom_bmp, rect[0] + 15, rect[1] + 15, 0.98, Some(50), Some(50));

                let mut temp_str = it.label.as_ref().to_string();
                temp_str.truncate(8);
                let mut string_length = get_advance_string(&temp_str, 23.0);
                draw_string(&mut app_storage.circuit_element_canvas.canvas, &temp_str, rect[0]-8 + rect[2]/2 - string_length/2, rect[1]-5, C4_WHITE, 23.0);

                if in_rect(mouseinfo.x, mouseinfo.y, _rect){
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, rect, C4_YELLOW, false);
                    if mouseinfo.lclicked(){
                        app_storage.create_custom_circuit = true;
                        app_storage.custom_circuit_cursor = index as usize;
                    }
                    if mouseinfo.rclicked() 
                    && app_storage.teacher_mode {
                        app_storage.panel_custom_circuit = true;
                        app_storage.custom_circuit_cursor = index as usize;
                    }
                }
                index += 1;
            }

            if app_storage.custom_circuit_cursor < app_storage.custom_circuit_elements.len()
            && app_storage.create_custom_circuit {
                app_storage.selected_circuit_element = SelectedCircuitElement::Custom;
                app_storage.create_custom_circuit = false;
            }

            if app_storage.teacher_mode {
                //TODO check if there are less that 10 elements
                let rect = [80*(index%2)+5, subcanvas_h - 70*((index+2)/2) - 5, 80, 70];
                let add_x_offset = 25;
                let add_y_offset = 5;
                let _rect = [rect[0]-15+x_offset, rect[1]-15 + y_offset, 80, 70];

                draw_char(&mut app_storage.circuit_element_canvas.canvas, '+', rect[0] + add_x_offset, rect[1]+add_y_offset, C4_WHITE, 50.0);
                if in_rect(mouseinfo.x, mouseinfo.y, _rect){
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas,  rect, C4_YELLOW, false);
                    if mouseinfo.lclicked(){
                        app_storage.create_custom_circuit = true;

                        let mut circuit_element = CircuitElement::empty();
                        circuit_element.label.copystr(&format!("Custom {}", index));

                        app_storage.custom_circuit_elements.push(circuit_element);
                        app_storage.custom_circuit_cursor = index as usize;
                    }
                }


                let mut remove_element = false;
                if app_storage.panel_custom_circuit
                && app_storage.custom_circuit_cursor < app_storage.custom_circuit_elements.len(){
                    let properties_rect = [subcanvas_w+20+x_offset, y_offset, PROPERTIES_W, subcanvas_h];
                    let mut prop_y_offset = subcanvas_h + y_offset - 30;

                    draw_rect(&mut os_package.window_canvas, properties_rect, C4_BLACK, true);
                    draw_string(&mut os_package.window_canvas, "Custom Element Builder", properties_rect[0]+2, prop_y_offset, C4_WHITE, 28.0);
                    prop_y_offset -= 30;

                    {//Exit properties
                        let char_x_len = get_advance('x', 26.0);
                        let button_x = properties_rect[0] + properties_rect[2] - get_advance('x', 26.0) - 8;
                        let button_y = properties_rect[1] + properties_rect[3] - 26;

                        let mut color = C4_WHITE;

                        if in_rect(mouseinfo.x, mouseinfo.y, [button_x, button_y, char_x_len+5, 26]){
                            color = C4_RED;
                            if mouseinfo.lclicked(){
                                app_storage.panel_custom_circuit = false;
                            }
                        }
                        draw_char(&mut os_package.window_canvas, 'x', button_x, button_y, color, 26.0);
                    }

                    let it = &mut app_storage.custom_circuit_elements[app_storage.custom_circuit_cursor];
                    
                    let c_textbox = &mut app_storage.custom_circuit_textbox;

                    let lstr_len = draw_string(&mut os_package.window_canvas, "Label: ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                    draw_string(&mut os_package.window_canvas, it.label.as_ref(), properties_rect[0] + lstr_len, prop_y_offset, C4_WHITE, PANEL_FONT);

                    {//
                        c_textbox.label_textbox.x = properties_rect[0] + lstr_len;
                        c_textbox.label_textbox.y = prop_y_offset;

                        c_textbox.label_textbox.text_buffer.clear();
                        c_textbox.label_textbox.text_buffer.push_str(it.label.as_ref());

                        c_textbox.label_textbox.update(keyboardinfo, textinfo, mouseinfo);
                        c_textbox.label_textbox.draw(&mut os_package.window_canvas,  app_storage.timer.elapsed().as_secs_f32());

                        //Sync
                        //TODO we will run into problems if custom names overlap 
                        for jt in app_storage.arr_circuit_elements.iter_mut(){
                            if it.label == jt.label
                            && (jt.circuit_element_type == SelectedCircuitElement::Custom 
                                || jt.circuit_element_type == SelectedCircuitElement::CustomVoltmeter
                                || jt.circuit_element_type == SelectedCircuitElement::CustomAmmeter){
                                jt.label.copystr(&c_textbox.label_textbox.text_buffer);
                            }
                        }
                        it.label.copystr(&c_textbox.label_textbox.text_buffer);
                    }

                    prop_y_offset -= (PANEL_FONT + 2f32) as i32;

                    {//Select type of custom element
                        use ui_tools::*;

                        let mut cl_x_offset = properties_rect[0];
                        let mut custom_bg = C4_DGREY;
                        let mut voltme_bg = C4_DGREY;
                        let mut ammete_bg = C4_DGREY;

                        let font_size = 18f32;


                        if it.circuit_element_type == SelectedCircuitElement::Custom{
                            custom_bg = C4_GREY;
                        } else if it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter {
                            voltme_bg = C4_GREY;
                        } else if it.circuit_element_type == SelectedCircuitElement::CustomAmmeter {
                            ammete_bg = C4_GREY;
                        }
                        
                        {
                            set_button_bg_color1(custom_bg);
                            let brt = basic_button( &mut os_package.window_canvas, "Custom", cl_x_offset, prop_y_offset, font_size, mouseinfo );
                            cl_x_offset += brt.rect[2];

                            if brt.lclicked {
                                it.circuit_element_type = SelectedCircuitElement::Custom;
                            }
                        }
                        {
                            set_button_bg_color1(voltme_bg);
                            let brt = basic_button( &mut os_package.window_canvas, "Custom Voltmeter", cl_x_offset, prop_y_offset, font_size, mouseinfo );
                            cl_x_offset += brt.rect[2];

                            if brt.lclicked {
                                it.circuit_element_type = SelectedCircuitElement::CustomVoltmeter;
                            }
                        }
                        {
                            set_button_bg_color1(ammete_bg);
                            let brt = basic_button( &mut os_package.window_canvas, "Custom Ammeter", cl_x_offset, prop_y_offset, font_size, mouseinfo );
                            cl_x_offset += brt.rect[2];

                            if brt.lclicked {
                                it.circuit_element_type = SelectedCircuitElement::CustomAmmeter;
                            }
                        }
                        reset_button_bg_color1();
                        
                    }
                    prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                    prop_y_offset -= (PANEL_FONT + 2f32) as i32;

                    fn do_text_box_things( tb: &mut TextBox, properties_x: i32, properties_y: i32,
                                           textinfo: &TextInfo, mouseinfo: &MouseInfo, keyboardinfo: &KeyboardInfo,
                                           it_property: &mut f32, window_canvas: &mut WindowCanvas, time: f32,
                                           ){

                        tb.x = properties_x;
                        tb.y = properties_y;

                        let tb_previously_active = tb.active;

                        tb.update(keyboardinfo, textinfo, mouseinfo);

                        if keyboardinfo.key.contains(&KeyboardEnum::Enter) {
                            match tb.text_buffer.parse::<f32>(){
                                Ok(parsed_f32)=>{ *it_property = parsed_f32; },
                                Err(_)=>{},
                            }
                        }
                        if tb.active{
                            
                        } else {
                            if tb_previously_active {
                                match tb.text_buffer.parse::<f32>(){
                                    Ok(parsed_f32)=>{ *it_property = parsed_f32; },
                                    Err(_)=>{},
                                }
                            }

                            tb.text_buffer.clear();
                            tb.text_cursor = 0;
                            tb.text_buffer += &format!("{:.2}", it_property);
                        }
                        tb.draw(window_canvas, time);
                    }



                    if it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter
                    || it.circuit_element_type == SelectedCircuitElement::CustomAmmeter{
                        use ui_tools::*;
                        {
                            let rstr_len = draw_string(&mut os_package.window_canvas, "Bias: ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.bias_textbox, properties_rect[0] + rstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.bias, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );

                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                            if it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter{
                                it.resistance = VOLTMETER_RESISTANCE;
                            }
                        }
                        {
                            let rstr_len = draw_string(&mut os_package.window_canvas, "Noise: ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.noise_textbox, properties_rect[0] + rstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.noise, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );

                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }


                        {
                            let rstr_len = draw_string(&mut os_package.window_canvas, "Drift: ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.drift_textbox, properties_rect[0] + rstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.drift, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );

                            if it.drift > 1f32 {
                                it.drift = 1f32;
                            } else if it.drift < 0f32 {
                                it.drift = 0f32;
                            }
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                            draw_string(&mut os_package.window_canvas, "d := drift,  p_m := previous_measurement", properties_rect[0], prop_y_offset, C4_WHITE, 18f32);
                            prop_y_offset -= (18f32 + 2f32) as i32;
                            draw_string(&mut os_package.window_canvas, "m := current nominal measurement", properties_rect[0], prop_y_offset, C4_WHITE, 18f32);
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;

                            draw_string(&mut os_package.window_canvas, "measurement :=", properties_rect[0], prop_y_offset, C4_WHITE, 18f32);
                            prop_y_offset -= (18f32 + 2f32) as i32;
                            draw_string(&mut os_package.window_canvas, "d * p_m + (1-d) * (m +bias)  + sample_N(noise)", properties_rect[0], prop_y_offset, C4_WHITE, 18f32);
                            prop_y_offset -= (18f32 + 2f32) as i32;
                            draw_string(&mut os_package.window_canvas, "A good drift is 0.8.", properties_rect[0], prop_y_offset, C4_WHITE, 18f32);
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }


                    } else if it.circuit_element_type == SelectedCircuitElement::Custom{

                        {
                            let rstr_len = draw_string(&mut os_package.window_canvas, "Resistance(Ω): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.resistance_textbox, properties_rect[0] + rstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.resistance, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );

                            let _len = properties_rect[0]+rstr_len+c_textbox.resistance_textbox.max_render_length;
                            let pm_len = draw_string(&mut os_package.window_canvas, "+ ", _len,
                                                     prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.unc_resistance_textbox, _len + pm_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.unc_resistance, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }

                        {
                            let vstr_len = draw_string(&mut os_package.window_canvas, "Voltage(V): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.voltage_textbox, properties_rect[0] + vstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.voltage, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );

                            let _len = properties_rect[0]+vstr_len+c_textbox.voltage_textbox.max_render_length;
                            let pm_len = draw_string(&mut os_package.window_canvas, "+ ", _len,
                                                     prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.unc_voltage_textbox, _len + pm_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.unc_voltage, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }

                        //{
                        //    let cstr_len = draw_string(&mut os_package.window_canvas, "Current(A): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                        //    do_text_box_things( &mut c_textbox.current_textbox, properties_rect[0] + cstr_len, prop_y_offset,
                        //                           textinfo, mouseinfo, keyboardinfo,
                        //                           &mut it.current, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                        //                           );
                        //    let _len = properties_rect[0]+cstr_len+c_textbox.current_textbox.max_render_length;
                        //    let pm_len = draw_string(&mut os_package.window_canvas, "±: ", _len,
                        //                             prop_y_offset, C4_WHITE, PANEL_FONT);
                        //    do_text_box_things( &mut c_textbox.unc_current_textbox, _len + pm_len, prop_y_offset,
                        //                           textinfo, mouseinfo, keyboardinfo,
                        //                           &mut it.unc_current, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                        //                           );
                        //    prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        //}

                        {
                            let cstr_len = draw_string(&mut os_package.window_canvas, "Capacitance(F): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.capacitance_textbox, properties_rect[0] + cstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.capacitance, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            let _len = properties_rect[0]+cstr_len+c_textbox.capacitance_textbox.max_render_length;
                            let pm_len = draw_string(&mut os_package.window_canvas, "+ ", _len,
                                                     prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.unc_capacitance_textbox, _len + pm_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.unc_capacitance, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }

                        {
                            let istr_len = draw_string(&mut os_package.window_canvas, "Inductance(H): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.inductance_textbox, properties_rect[0] + istr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.inductance, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            let _len = properties_rect[0]+istr_len+c_textbox.inductance_textbox.max_render_length;
                            let pm_len = draw_string(&mut os_package.window_canvas, "+ ", _len,
                                                     prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.unc_inductance_textbox, _len + pm_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.unc_inductance, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }

                        {
                            let cstr_len = draw_string(&mut os_package.window_canvas, "Charge(C): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.charge_textbox, properties_rect[0] + cstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.charge, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            let _len = properties_rect[0]+cstr_len+c_textbox.charge_textbox.max_render_length;
                            let pm_len = draw_string(&mut os_package.window_canvas, "+ ", _len,
                                                     prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.unc_charge_textbox, _len + pm_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.unc_charge, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }

                        {
                            let mfstr_len = draw_string(&mut os_package.window_canvas, "Flux(Wb): ", properties_rect[0], prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.magflux_textbox, properties_rect[0] + mfstr_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.magnetic_flux, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            let _len = properties_rect[0]+mfstr_len+c_textbox.magflux_textbox.max_render_length;
                            let pm_len = draw_string(&mut os_package.window_canvas, "+ ", _len,
                                                     prop_y_offset, C4_WHITE, PANEL_FONT);
                            do_text_box_things( &mut c_textbox.unc_magflux_textbox, _len + pm_len, prop_y_offset,
                                                   textinfo, mouseinfo, keyboardinfo,
                                                   &mut it.unc_magnetic_flux, &mut os_package.window_canvas, app_storage.timer.elapsed().as_secs_f32(),
                                                   );
                            prop_y_offset -= (PANEL_FONT + 2f32) as i32;
                        }
                    }
                    //////////
                    //Syncing
                    for jt in app_storage.arr_circuit_elements.iter_mut(){
                        if it.label == jt.label
                        && jt.circuit_element_type == SelectedCircuitElement::Custom{
                            jt.resistance = it.resistance;
                            jt.voltage    = it.voltage;
                            jt.current    = it.current;
                            jt.capacitance= it.capacitance;
                            jt.inductance = it.inductance;
                            jt.charge     = it.charge;
                            jt.magnetic_flux = it.magnetic_flux;

                            jt.unc_resistance    = sample_normal(it.unc_resistance);
                            jt.unc_voltage       = sample_normal(it.unc_voltage);
                            jt.unc_current       = sample_normal(it.unc_current);
                            jt.unc_capacitance   = sample_normal(it.unc_capacitance);
                            jt.unc_inductance    = sample_normal(it.unc_inductance);
                            jt.unc_charge        = sample_normal(it.unc_charge);
                            jt.unc_magnetic_flux = sample_normal(it.unc_magnetic_flux);

                            jt.bias  = 0f32;
                            jt.noise = 0f32;
                            jt.drift = 0f32;
                        }
                        if it.label == jt.label
                        && jt.circuit_element_type == SelectedCircuitElement::CustomVoltmeter{
                            jt.resistance = VOLTMETER_RESISTANCE;
                            jt.voltage    = 0f32;
                            jt.current    = 0f32;
                            jt.capacitance= 0f32;
                            jt.inductance = 0f32;
                            jt.charge     = 0f32;
                            jt.magnetic_flux = 0f32;

                            jt.unc_resistance    = 0f32;
                            jt.unc_voltage       = 0f32;
                            jt.unc_current       = 0f32;
                            jt.unc_capacitance   = 0f32;
                            jt.unc_inductance    = 0f32;
                            jt.unc_charge        = 0f32;
                            jt.unc_magnetic_flux = 0f32;

                            jt.bias  = it.bias;
                            jt.noise = it.noise;
                            jt.drift = it.drift;
                        }
                        if it.label == jt.label
                        && jt.circuit_element_type == SelectedCircuitElement::CustomAmmeter{
                            jt.resistance = WIRE_RESISTANCE;
                            jt.voltage    = 0f32;
                            jt.current    = 0f32;
                            jt.capacitance= 0f32;
                            jt.inductance = 0f32;
                            jt.charge     = 0f32;
                            jt.magnetic_flux = 0f32;

                            jt.unc_resistance    = 0f32;
                            jt.unc_voltage       = 0f32;
                            jt.unc_current       = 0f32;
                            jt.unc_capacitance   = 0f32;
                            jt.unc_inductance    = 0f32;
                            jt.unc_charge        = 0f32;
                            jt.unc_magnetic_flux = 0f32;

                            jt.bias  = it.bias;
                            jt.noise = it.noise;
                            jt.drift = it.drift;
                        }
                    }
                    ////////
                    {//Delete element

                        let _w = get_advance_string("Delete", PANEL_FONT);

                        let d_w = _w + 16;
                        let d_x = properties_rect[0] + properties_rect[2] - d_w - 5;
                        let d_h = PANEL_FONT as i32 + 2;
                        let d_y = properties_rect[1] + 2;

                        let rect = [d_x, d_y, d_w, d_h];

                        draw_rect(&mut os_package.window_canvas, rect, C4_DGREY, true);
                        if in_rect(mouseinfo.x, mouseinfo.y, rect) {
                            draw_rect(&mut os_package.window_canvas, rect, C4_LGREY, true);
                            if mouseinfo.lclicked() {
                                remove_element = true;
                            }
                        }
                        draw_string(&mut os_package.window_canvas, "Delete", d_x + d_w / 2 - _w / 2 - 5, d_y, C4_LGREY, PANEL_FONT);
                    }
                }
                if remove_element {
                    app_storage.custom_circuit_elements.remove(app_storage.custom_circuit_cursor);
                    app_storage.create_custom_circuit = false;
                }
            }


        }
        else if app_storage.circuit_menu_type == CircuitMenuType::Default{

            draw_rect(&mut app_storage.circuit_element_canvas.canvas, [0, 0, subcanvas_w, subcanvas_h], C4_WHITE, true);
            draw_rect(&mut app_storage.circuit_element_canvas.canvas, [3, 3, subcanvas_w-6, subcanvas_h-6], COLOR_MENU_BKG, true);



            //TODO remake assets
            let alter_offset_y = 10;//TODO BAD

            ///////////////////////
            { // Resistor
                let resistor_xy = [20, 280+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.resistor_bmp, resistor_xy[0], resistor_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Resistor", resistor_xy[0] - 10, resistor_xy[1] - 15, COLOR_TEXT, 23.0);

                let _rect = [resistor_xy[0]-15+x_offset, resistor_xy[1]-15 + y_offset, 80, 70];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect){
                    //NOTE Resistor
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [resistor_xy[0]-15, resistor_xy[1]-15, 80, 70], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Resistor;
                    }
                }
            }
            { // Voltmeter
                let voltm_xy = [110, 280+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.voltmeter_bmp, voltm_xy[0], voltm_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Voltmeter", voltm_xy[0]-15, voltm_xy[1]-15, COLOR_TEXT, 23.0);

                let _rect = [voltm_xy[0]-15+x_offset, voltm_xy[1]-15+y_offset, 80, 70];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [voltm_xy[0]-15, voltm_xy[1]-15, 80, 70], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Voltmeter;
                    }
                }
            }
            { //Wire
                let wire_xy = [20, 237+alter_offset_y];
                draw_rect(&mut app_storage.circuit_element_canvas.canvas, [wire_xy[0], wire_xy[1], 50, 2], C4_WHITE, true);
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Wire", wire_xy[0]+5, wire_xy[1]-45+6, COLOR_TEXT, 23.0);

                let _rect = [wire_xy[0]-15+x_offset, wire_xy[1]-35+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [wire_xy[0]-15, wire_xy[1]-35, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Wire;
                    }
                }
            }
            {//Ammeter
                let ammeter_xy = [110, 212+alter_offset_y];

                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.ammeter_bmp, ammeter_xy[0], ammeter_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Ammeter", ammeter_xy[0]-15, ammeter_xy[1]-20+5, COLOR_TEXT, 23.0);

                let _rect = [ammeter_xy[0]-15+x_offset, ammeter_xy[1]-10+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [ammeter_xy[0]-15, ammeter_xy[1]-10, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Ammeter;
                    }
                }
            }
            {//Battery
                let battery_xy = [20, 145+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.battery_bmp, battery_xy[0], battery_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Battery", battery_xy[0]-5, battery_xy[1]-15, COLOR_TEXT, 23.0);

                let _rect = [battery_xy[0]-15+x_offset, battery_xy[1]-15+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [battery_xy[0]-15, battery_xy[1]-15, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Battery;
                    }
                }
            }
            {//Inductor
                let inductor_xy = [110, 145+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.inductor_bmp, inductor_xy[0], inductor_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Inductor", inductor_xy[0]-12, inductor_xy[1]-15, COLOR_TEXT, 23.0);

                let _rect = [inductor_xy[0]-15+x_offset, inductor_xy[1]-15+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [inductor_xy[0]-15, inductor_xy[1]-15, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Inductor;
                    }
                }
            }
            {//Capacitor
                let capacitor_xy = [20, 75+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.capacitor_bmp, capacitor_xy[0], capacitor_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Capacitor", capacitor_xy[0]-15, capacitor_xy[1]-15, COLOR_TEXT, 23.0);

                let _rect = [capacitor_xy[0]-15+x_offset, capacitor_xy[1]-15+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [capacitor_xy[0]-15, capacitor_xy[1]-15, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Capacitor;
                    }
                }
            }
            {//Switch
                let switch_xy = [110, 75+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.switch_open_bmp, switch_xy[0], switch_xy[1], 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "Switch", switch_xy[0]-5, switch_xy[1]-15, COLOR_TEXT, 23.0);

                let _rect = [switch_xy[0]-15+x_offset, switch_xy[1]-15+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [switch_xy[0]-15, switch_xy[1]-15, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::Switch;
                    }
                }
            }
            {//AC
                let ac_xy = [20, 5+alter_offset_y];
                draw_bmp(&mut app_storage.circuit_element_canvas.canvas, &app_storage.ac_bmp, ac_xy[0], ac_xy[1]+5, 0.98, Some(50), Some(50));
                draw_string(&mut app_storage.circuit_element_canvas.canvas, "AC", ac_xy[0]+10, ac_xy[1]-13, COLOR_TEXT, 23.0);

                let _rect = [ac_xy[0]-15+x_offset, ac_xy[1]-15+y_offset, 80, 65];
                if in_rect(mouseinfo.x, mouseinfo.y, _rect ){
                    
                    draw_rect(&mut app_storage.circuit_element_canvas.canvas, [ac_xy[0]-15, ac_xy[1]-10, 80, 65], C4_YELLOW, false);
                    if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down {
                        app_storage.selected_circuit_element = SelectedCircuitElement::AC;
                    }
                }
            }

        }
        draw_subcanvas(&mut os_package.window_canvas, &app_storage.circuit_element_canvas, x_offset, y_offset, 1.0);
    }
    //////////////////////////////////////////////////////////////////////

    let mut subcanvas_x = 0;
    {//Moving subcanvas
        let mut x = os_package.window_canvas.w - app_storage.menu_canvas.canvas.w;
        let mut short_cut_pressed = 0;
        for it in keyboardinfo.key.iter(){
            if *it == KeyboardEnum::A  
            || *it == KeyboardEnum::Ctrl{ //TODO we need to change how keyboard info works
                short_cut_pressed += 1;
            }
        }
        if short_cut_pressed == 2 
        && app_storage.menu_move_activated_time == 0{

            app_storage.menu_move_activated = true;
            app_storage.menu_move_activated_time =  app_storage.timer.elapsed().as_micros();
        }

        let rect_main_canvas = [window_w-25, window_h-26, 20, 20];
        let rect_sub_  = [x+5, temp_h-25, 20, 20];
        let rect_sub   = [  5, temp_h-25, 20, 20];   


        let mut color = C4_WHITE;
        if app_storage.menu_offscreen {
            if in_rect(mouseinfo.x, mouseinfo.y, rect_main_canvas) {
                color = C4_LGREY;
                if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down{

                    app_storage.menu_move_activated = true;
                    app_storage.menu_move_activated_time =  app_storage.timer.elapsed().as_micros();

                }
            }
        } else {
            if in_rect(mouseinfo.x, mouseinfo.y, rect_sub_) {  //NOTE if subcanvas is on screen
                color = C4_LGREY;
                if mouseinfo.lbutton == ButtonStatus::Up && mouseinfo.old_lbutton == ButtonStatus::Down{

                    app_storage.menu_move_activated = true;
                    app_storage.menu_move_activated_time =  app_storage.timer.elapsed().as_micros();

                }
            }
        }

        for i in 1..4{
            let y = (5.0 * i as f32) as i32 + rect_main_canvas[1];
            let _rect = [rect_main_canvas[0], y, rect_main_canvas[2], 2];
            draw_rect(&mut os_package.window_canvas, _rect, color, true);
        }

        for i in 1..4{
            let y = (5.0 * i as f32) as i32 + rect_sub[1];
            let _rect = [rect_sub[0], y, rect_sub[2], 2];
            draw_rect(&mut app_storage.menu_canvas.canvas, _rect, color, true);
        }


        let delta_max_time = 0.2E6;
        if app_storage.menu_move_activated {

            //let frac = global_time_delta.as_sec_f32() / Duration;::from_millis(16).as_sec_f32(); 
            let mut delta = (app_storage.timer.elapsed().as_micros() - app_storage.menu_move_activated_time) as f32;
            x = ((os_package.window_canvas.w - app_storage.menu_canvas.canvas.w) as f32 * ((delta_max_time + delta) / delta_max_time)) as i32; //TODO

            if app_storage.menu_offscreen {
                x = ((window_w as f32 - (delta/delta_max_time) * (window_w as f32 / 2.0)) as i32).max(os_package.window_canvas.w - app_storage.menu_canvas.canvas.w); //TODO
            }

            if delta.abs() > delta_max_time{
                app_storage.menu_move_activated = false;
                app_storage.menu_move_activated_time = 0;
                app_storage.menu_offscreen = !app_storage.menu_offscreen;  //TODO this is kinda trash will need to rewrite

            }
        }

        if app_storage.menu_offscreen && app_storage.menu_move_activated == false {
            x = window_w;
        }


        subcanvas_x = x;
    }


    //NOTE save and load circuit
    {
        let mut icon = &app_storage.save_icon_bmp;
        let mut icon_rect = [window_w-65, window_h-28, 26, 26];
        let mut _icon_rect = [window_w-65, window_h-28, 26, 26];
        let mut rect = [window_w-icon_rect[2]-100, icon_rect[1]-35, 110, 31];
        let mut canvas;
        draw_bmp(&mut os_package.window_canvas, icon, icon_rect[0], icon_rect[1], 0.98, Some(icon_rect[2]), Some(icon_rect[3]));

        draw_bmp(&mut app_storage.menu_canvas.canvas, icon, 30, icon_rect[1], 0.98, Some(icon_rect[2]), Some(icon_rect[3]));
        if !app_storage.menu_offscreen{
            icon_rect = [subcanvas_x+30, window_h-28, 26, 26];
            _icon_rect = [30, window_h-28, 26, 26];
            rect = [0, icon_rect[1]-35, 110, 31];
            canvas = &mut app_storage.menu_canvas.canvas;
        } else {
            canvas = &mut os_package.window_canvas;
        }
        //if app_storage.menu_offscreen {
        {
        
            if in_rect(mouseinfo.x, mouseinfo.y, icon_rect){
                icon = &app_storage.save_icon_bmp_alt;
                if mouseinfo.lclicked() {
                    app_storage.save_toggle = !app_storage.save_toggle;
                    if app_storage.save_toggle == false {
                        app_storage.save_toggle_saveload = SaveLoadEnum::Default;
                    }
                }
                draw_bmp(canvas, icon, _icon_rect[0], _icon_rect[1], 0.98, Some(icon_rect[2]), Some(icon_rect[3]));
            } 

            if app_storage.save_toggle{
                let mut previously_saved_circuits = vec![];

                for it in std::fs::read_dir("./").unwrap(){//TODO why so many layers deep this is stupid
                    if it.is_ok(){
                        let ref_it = it.as_ref().unwrap().path();
                        if ref_it.is_file(){
                            match ref_it.extension(){
                                Some(temp_str)=>{
                                    if temp_str == "cd"{
                                        previously_saved_circuits.push(ref_it.to_str().unwrap().to_string());
                                    }
                                },
                                _=>{}
                            }
                        }
                    }
                }
                match app_storage.save_toggle_saveload{
                    SaveLoadEnum::Save=>{


                        let save_menu_font = 26.0;
                        let mut save_menu_rect = [window_w-300, rect[1]-save_menu_font as i32 * previously_saved_circuits.len() as i32, 300, 
                                              save_menu_font as i32 * ( 1 + previously_saved_circuits.len() as i32 )];
                        if !app_storage.menu_offscreen {
                            save_menu_rect[0] = 0;
                        }

                        draw_rect(canvas, save_menu_rect, C4_BLACK, true);
                        draw_string(canvas, "Save:", save_menu_rect[0]+4, save_menu_rect[1] + save_menu_rect[3] - save_menu_font as i32, C4_WHITE, save_menu_font);

                        for (i, it) in previously_saved_circuits.iter().enumerate(){
                            let y = save_menu_rect[1] + save_menu_rect[3] - (i + 2) as i32 * save_menu_font as i32 - 2;
                            let sub_rect = [save_menu_rect[0], y, save_menu_rect[2], save_menu_font as i32];
                            let _sub_rect = if app_storage.menu_offscreen{ [save_menu_rect[0], y, save_menu_rect[2], save_menu_font as i32]}
                                          else { [subcanvas_x+save_menu_rect[0], y, save_menu_rect[2], save_menu_font as i32] };

                            if in_rect(mouseinfo.x, mouseinfo.y, _sub_rect){
                                draw_rect(canvas, sub_rect, C4_GREY, true);
                                if mouseinfo.lclicked(){
                                    app_storage.save_textbox.text_buffer = it.to_string();
                                }
                            }

                            draw_string(canvas, it, save_menu_rect[0]+4, y, C4_WHITE, save_menu_font);
                        }
                        if !app_storage.menu_offscreen { app_storage.save_textbox.offset_x = subcanvas_x;}
                        app_storage.save_textbox.x = save_menu_rect[0]+50;
                        app_storage.save_textbox.y = save_menu_rect[1] + save_menu_rect[3] - save_menu_font as i32;
                        app_storage.save_textbox.omega = 4.0;

                        app_storage.save_textbox.update(keyboardinfo, textinfo, mouseinfo);
                        app_storage.save_textbox.draw(canvas, app_storage.timer.elapsed().as_secs_f32());

                        if app_storage.arr_circuit_elements.len() != 0 
                        && keyboardinfo.key.contains(&KeyboardEnum::Enter) {
                            save_circuit_diagram(&app_storage.save_textbox.text_buffer, &app_storage.arr_circuit_elements);
                            app_storage.save_toggle_saveload = SaveLoadEnum::Default;

                            app_storage.messages.push( (MessageType::Default, format!("Message: {} saved.", &app_storage.save_textbox.text_buffer)) );
                            app_storage.save_toggle = false;
                        }
                    },
                    SaveLoadEnum::Load=>{
                        let load_menu_font = 26.0;
                        let mut load_menu_rect = [window_w-300, rect[1] - load_menu_font as i32 * previously_saved_circuits.len() as i32, 300, load_menu_font as i32 * ( 1 + previously_saved_circuits.len() as i32 )];
                        if !app_storage.menu_offscreen {
                            load_menu_rect[0] = 0;
                        }

                        draw_rect(canvas, load_menu_rect, C4_BLACK, true);
                        draw_string(canvas, "Load:", load_menu_rect[0]+4, load_menu_rect[1] + load_menu_rect[3] - load_menu_font as i32, C4_WHITE, load_menu_font);

                        let mut temp_string  = String::new();
                        for (i, it) in previously_saved_circuits.iter().enumerate(){
                            let y = load_menu_rect[1] + load_menu_rect[3] - (i + 2) as i32 * load_menu_font as i32 - 2;
                            let sub_rect = [load_menu_rect[0], y, load_menu_rect[2], load_menu_font as i32];
                            let _sub_rect = if app_storage.menu_offscreen{ [load_menu_rect[0], y, load_menu_rect[2], load_menu_font as i32]}
                                          else { [subcanvas_x+load_menu_rect[0], y, load_menu_rect[2], load_menu_font as i32] };
                            if in_rect(mouseinfo.x, mouseinfo.y, _sub_rect){
                                draw_rect(canvas, sub_rect, C4_GREY, true);
                                if mouseinfo.lclicked(){
                                    temp_string = it.to_string();
                                }
                            }
                            draw_string(canvas, it, load_menu_rect[0]+4, y, C4_WHITE, load_menu_font);
                        }
                        if previously_saved_circuits.contains(&temp_string) {

                            let v = load_circuit_diagram(&temp_string);
                            app_storage.arr_circuit_elements = v;
                            
                            app_storage.saved_circuit_volts.clear();
                            app_storage.saved_circuit_currents.clear();
                            app_storage.circuit_textbox_hash.clear();

                            for it in app_storage.arr_circuit_elements.iter(){
                                app_storage.circuit_textbox_hash.insert((it.unique_a_node, it.unique_b_node), CircuitElementTextBox::new());

                                if it.circuit_element_type == SelectedCircuitElement::Voltmeter
                                || it.circuit_element_type == SelectedCircuitElement::CustomVoltmeter{
                                    app_storage.saved_circuit_volts.insert((it.unique_a_node, it.unique_b_node), [Vec::new(), Vec::new()]);
                                }
                                if it.circuit_element_type == SelectedCircuitElement::Ammeter
                                || it.circuit_element_type == SelectedCircuitElement::CustomAmmeter{
                                    app_storage.saved_circuit_currents.insert((it.unique_a_node, it.unique_b_node), [Vec::new(), Vec::new()]);
                                }
                            }

                            app_storage.save_toggle_saveload = SaveLoadEnum::Default;
                            app_storage.save_toggle = false;
                        }
                    },
                    SaveLoadEnum::Default=>{
                        draw_rect(canvas, rect, C4_BLACK, true);

                        let mut save_color = C4_GREY;
                        let save_rect  = [rect[0]+4, rect[1]+4, 50, 24];
                        let mut _save_rect  = [rect[0]+4, rect[1]+4, 50, 24];
                        if !app_storage.menu_offscreen {
                            _save_rect[0]  = rect[0]+4 + subcanvas_x;
                        }

                        if in_rect(mouseinfo.x, mouseinfo.y, _save_rect){
                            save_color = C4_LGREY;
                            if mouseinfo.lclicked(){
                                app_storage.save_toggle_saveload = SaveLoadEnum::Save;
                            }
                        }
                        draw_rect(canvas, save_rect, save_color, true);
                        draw_string(canvas, "Save", save_rect[0], save_rect[1]-2, C4_WHITE, 24.0);



                        let mut load_color = C4_GREY;
                        let load_rect  = [rect[0]+57, rect[1]+4, 50, 24];
                        let mut _load_rect  = [rect[0]+57, rect[1]+4, 50, 24];
                        if !app_storage.menu_offscreen {
                            _load_rect[0]  = rect[0]+ 57 + subcanvas_x;
                        }

                        if in_rect(mouseinfo.x, mouseinfo.y, _load_rect){
                            load_color = C4_LGREY;
                            if mouseinfo.lclicked(){
                                app_storage.save_toggle_saveload = SaveLoadEnum::Load;
                            }
                        }
                        draw_rect(canvas, load_rect, load_color, true);
                        draw_string(canvas, "Load", load_rect[0], load_rect[1]-2, C4_WHITE, 24.0);
                    },
                }
            }
        }
    }


    {//error report
        let messages = &mut app_storage.messages;
        let message_index = &mut app_storage.message_index;
        let message_stopwatch = &mut app_storage.message_timer;

        if messages.len() > 0 {
            let index = match message_index {
                Some(_index)=>{
                    let id = *_index;

                    if id < messages.len() {
                        let mut message_onscreen_duration = DEFAULT_MESSAGE_ONSCREEN_DURATION;
                        if messages[id].0 == MessageType::Error{
                            message_onscreen_duration = ERROR_MESSAGE_ONSCREEN_DURATION;
                        }
                        if message_stopwatch.get_time() >= message_onscreen_duration {
                            *message_index =  Some(id + 1);
                            message_stopwatch.reset();
                        }
                    }
                    id
                },
                None=>{
                    *message_index = Some(0);
                    message_stopwatch.reset();
                    0 
                }
            };
            if index >= messages.len() {
                messages.clear();
                *message_index = None;
            } else{
                let rect = [0, 0, window_w, PANEL_FONT as i32 + 8];
                let alpha = 
                { 
                    let mut x = message_stopwatch.get_time().as_millis() as f32 * 0.001;
                    let max_duration = if messages[index].0 == MessageType::Default {
                                        DEFAULT_MESSAGE_ONSCREEN_DURATION.as_millis() as f32 * 0.001
                                    } else {
                                        ERROR_MESSAGE_ONSCREEN_DURATION.as_millis() as f32 * 0.001
                                    };
                    let mut rt = x;
                    if x > max_duration - 1f32{
                        rt = max_duration - x ;
                    }
                    if rt > 1.0{
                        rt = 1.0;
                    } else if rt < 0f32 {
                        rt = 0.0;
                    }
                    rt
                };
                let mut color_rect = c3_to_c4(C3_MGREEN, alpha);
                if messages[index].0 == MessageType::Error{
                    color_rect = c3_to_c4(C3_RED, alpha);
                }
                draw_rect(&mut os_package.window_canvas, rect, color_rect, true);

                let mut text_pos = get_advance_string(&messages[index].1, panel_font);
                text_pos = window_w/2 - text_pos/2;
                change_font(FONT_NOTOSANS_BOLD);
                draw_string(&mut os_package.window_canvas, &messages[index].1, text_pos, 2, c3_to_c4(C3_WHITE, alpha.powi(2)), panel_font);
                change_font(FONT_NOTOSANS);
            }
        }
    }

    draw_subcanvas(&mut os_package.window_canvas, &app_storage.menu_canvas, subcanvas_x, 0, 1.0);




    {
        //app_storage.run_circuit = false;
        let mut circuit_has_been_altered = false;
        if copy_circuit_buffer.len() == app_storage.arr_circuit_elements.len(){
            for (i, it) in copy_circuit_buffer.iter().enumerate(){
                if it.x != app_storage.arr_circuit_elements[i].x
                || it.y != app_storage.arr_circuit_elements[i].y
                || it.orientation != app_storage.arr_circuit_elements[i].orientation{

                    circuit_has_been_altered = true;
                }
            }
        } else {
            circuit_has_been_altered = true;
        }

        //copy circuit buffer before buffer is updated then check if the positions and rotations are the same if not the circuit has been_altered.

        if circuit_has_been_altered 
        || app_storage.arr_circuit_elements.len() < 4{
            app_storage.run_circuit = false;

            for it in app_storage.arr_circuit_elements.iter_mut(){
                it.solved_current = None;
                it.solved_voltage = None;
                it.direction = None;
            }
        }
        {//TODO Draw simulate button
            let mut color = C4_DGREY;
            let rect =  [40, 115, 150, 40];
            if in_rect(mouseinfo.x, mouseinfo.y, rect){
                color = C4_GREY;
                if mouseinfo.lclicked() {
                    app_storage.run_circuit = !app_storage.run_circuit;

                    if app_storage.run_circuit 
                    && app_storage.timer_init == false{
                        app_storage.timer_init = true;
                        app_storage.stop_watch = 0f32;

                    } else if app_storage.run_circuit == false 
                    && app_storage.timer_init == true {

                        app_storage.stop_watch += TIME_STEP;
                        app_storage.stop_watch = 0f32;

                    } else if app_storage.run_circuit
                    && app_storage.timer_init == true{
                        app_storage.stop_watch = 0f32;
                    }
                }
            }
            if mouseinfo.lbutton == ButtonStatus::Down{
                color = C4_DGREY;
            }
            draw_rect(&mut os_package.window_canvas, rect, color, true);

            if app_storage.run_circuit {//TODO StopWatch
                app_storage.stop_watch += TIME_STEP;
            }

            if app_storage.run_circuit{//TODO timer
                let string_w = get_advance_string("Pause", 30.0);
                draw_string(&mut os_package.window_canvas, "Pause", rect[0]+rect[2]/2-string_w/2-4, rect[1]+4, C4_LGREY, 30.0);
            } else{
                let string_w = get_advance_string("Run", 30.0);
                draw_string(&mut os_package.window_canvas, "Run", rect[0]+rect[2]/2-string_w/2-4, rect[1]+4, C4_LGREY, 30.0);
            }
        }
        {//Clear screen button
            let rect = [40, 50, 150, 40];
            let mut bg_color = C4_DGREY;
            let txt_color = C4_LGREY;
            if in_rect(mouseinfo.x, mouseinfo.y, rect){
                bg_color = C4_GREY;
                if mouseinfo.lclicked(){
                    app_storage.selected_circuit_element =  SelectedCircuitElement::None;
                    app_storage.selected_circuit_element_orientation = 0f32;
                    app_storage.selected_circuit_properties = None;
                    app_storage.arr_circuit_elements.clear();
                }
            }
            draw_rect(&mut os_package.window_canvas, rect, bg_color, true);
            draw_string(&mut os_package.window_canvas, "Clear Board", rect[0]+5, rect[1]+4, txt_color, 30f32);
        }


        if app_storage.run_circuit 
        && app_storage.arr_circuit_elements.len() >= 4 //NOTE you can't have a circuit with less than 4 edges
        {
  
            //NOTE Updating the charge of capacitors and inductors
            app_storage.sim_time += TIME_STEP;
            for it in app_storage.arr_circuit_elements.iter_mut(){
                it.time += TIME_STEP;
                if it.circuit_element_type == SelectedCircuitElement::Custom{
                    //TODO this is redundant with the standard case. We should only be doing this thing once.
                    if it.capacitance.abs() > 0.00001f32
                    && it.solved_current.is_some(){
                        let current = if *it.direction.as_ref().unwrap() == CircuitElementDirection::AtoB{
                            it.solved_current.as_ref().unwrap().abs()
                        } else {
                            it.solved_current.as_ref().unwrap().abs() * -1f32
                        };

                        it.charge += current * TIME_STEP;
                    }
                    if it.inductance.abs() > 0.00001f32 
                    && it.solved_voltage.is_some(){
                        it.magnetic_flux += *it.solved_voltage.as_ref().unwrap() * TIME_STEP; //TODO i think this is wrong
                    } 
                }
                if it.circuit_element_type == SelectedCircuitElement::Capacitor 
                && it.solved_current.is_some(){
                    //TODO this is redundant with the custom case. We should only be doing this thing once.

                    let current = if *it.direction.as_ref().unwrap() == CircuitElementDirection::AtoB{
                        it.solved_current.as_ref().unwrap().abs()
                    } else {
                        it.solved_current.as_ref().unwrap().abs() * -1f32
                    };
                    it.charge += current * TIME_STEP;
                }
                if it.circuit_element_type == SelectedCircuitElement::Inductor 
                && it.solved_voltage.is_some(){
                    it.magnetic_flux += *it.solved_voltage.as_ref().unwrap() * TIME_STEP; //TODO i think this is wrong
                } 
                if it.circuit_element_type == SelectedCircuitElement::AC
                && it.solved_current.is_some(){//TODO does not work when you change max voltage or frequency
                    if it.ac_source_type == ACSourceType::Sin{
                        let max_voltage = it.max_voltage * PI;
                        let d_voltage2_over_dt2 = -1.0* ( 2f32*PI*it.frequency ).powi(2)*it.temp_step_voltage;

                        it.d_voltage_over_dt += d_voltage2_over_dt2 * TIME_STEP;
                        it.d_voltage_over_dt = it.d_voltage_over_dt.max(-1.0).min(1.0);


                        it.temp_step_voltage +=  it.d_voltage_over_dt * TIME_STEP;
                        it.temp_step_voltage  =  it.temp_step_voltage.max(-1.0/PI).min(1.0/PI);
                        it.voltage =  it.temp_step_voltage * max_voltage;
                        it.voltage =  it.max_voltage * (2f32 * PI * it.frequency * it.time).sin();

                    } else if it.ac_source_type == ACSourceType::Step{
                        let d_voltage2_over_dt2 = -1.0*( 2f32*PI*it.frequency ).powi(2)*it.temp_step_voltage/it.max_voltage;

                        it.d_voltage_over_dt += d_voltage2_over_dt2 * TIME_STEP;
                        it.d_voltage_over_dt = it.d_voltage_over_dt.max(-1.0).min(1.0);
                        let d_voltage_over_dt = it.d_voltage_over_dt;

                        it.temp_step_voltage +=  it.d_voltage_over_dt * TIME_STEP * it.max_voltage;
                        it.temp_step_voltage =  (2f32 * PI * it.frequency * it.time).sin();

                        it.voltage = if it.temp_step_voltage > 0f32 {  it.max_voltage } else { -1.0*it.max_voltage};
                    }
                }
            }


            for it in app_storage.arr_circuit_elements.iter_mut(){
                it.solved_current = None;
                it.solved_voltage = None;
                it.direction = None;

                //it.print_current = None;
                //it.print_voltage = None;

                if it.charge.is_nan() { 
                    it.charge = 0f32;
                }

                if it.magnetic_flux.is_nan() { 
                    it.magnetic_flux = 0f32;
                }
            }
            //TODO determine which elements are apart of a circuit
            for i in 0..app_storage.arr_circuit_elements.len(){
                let it = app_storage.arr_circuit_elements[i].clone();

                for j in i+1..app_storage.arr_circuit_elements.len(){
                    let jt = app_storage.arr_circuit_elements[j].clone();
                    //TODO handle bmps
                    let (it_x1, it_y1) = match it.circuit_element_type { 
                                          SelectedCircuitElement::Resistor  |
                                          SelectedCircuitElement::Battery   |
                                          SelectedCircuitElement::Inductor  |
                                          SelectedCircuitElement::Voltmeter |
                                          SelectedCircuitElement::Ammeter   |
                                          SelectedCircuitElement::Switch    |
                                          SelectedCircuitElement::Wire      |
                                          SelectedCircuitElement::AC        |
                                          SelectedCircuitElement::Custom    |
                                          SelectedCircuitElement::CustomVoltmeter  |
                                          SelectedCircuitElement::CustomAmmeter    |
                                          SelectedCircuitElement::Capacitor => { 
                                                                                 ////////////////
                                                                                 let _x = if it.orientation.sin().abs() < 0.001 {
                                                                                    it.x + ((it.orientation / 2f32).sin().abs() * (GRID_SIZE * 4) as f32) as i32
                                                                                 } else {
                                                                                    it.x + (it.orientation.sin().abs() * (GRID_SIZE * 2) as f32) as i32
                                                                                 };

                                                                                 let _y = if it.orientation.sin().abs() < 0.001 {
                                                                                    it.y + (it.orientation.cos().abs() * (GRID_SIZE * 2) as f32) as i32
                                                                                 } else {
                                                                                    it.y + (( (it.orientation - PI/ 2f32) / 2f32 ).sin().abs() * (GRID_SIZE * 4) as f32) as i32
                                                                                 };
                                                                                 (_x, _y)
                                                                                 ////////////////
                                                                                 //(it.x+(40 as f32 * it.orientation.sin().abs()) as i32, 
                                                                                 // it.y+(40 as f32 * it.orientation.cos().abs()) as i32) 
                                                                                },
                                          _=>{ panic!("ASDF"); } //TODO more informative panic
                                        };
                    let (jt_x1, jt_y1) = match jt.circuit_element_type { 
                                          SelectedCircuitElement::Resistor  |
                                          SelectedCircuitElement::Battery   | 
                                          SelectedCircuitElement::Inductor  | 
                                          SelectedCircuitElement::Voltmeter |
                                          SelectedCircuitElement::Ammeter   |
                                          SelectedCircuitElement::Switch    |
                                          SelectedCircuitElement::Wire      |
                                          SelectedCircuitElement::AC        |
                                          SelectedCircuitElement::Custom    |
                                          SelectedCircuitElement::CustomVoltmeter  |
                                          SelectedCircuitElement::CustomAmmeter    |
                                          SelectedCircuitElement::Capacitor => { 
                                                                                 let _x = if jt.orientation.sin().abs() < 0.001 {
                                                                                    jt.x + ((jt.orientation / 2f32).sin().abs() * (GRID_SIZE * 4) as f32) as i32
                                                                                 } else {
                                                                                    jt.x + (jt.orientation.sin().abs() * (GRID_SIZE * 2) as f32) as i32
                                                                                 };

                                                                                 let _y = if jt.orientation.sin().abs() < 0.001 {
                                                                                    jt.y + (jt.orientation.cos().abs() * (GRID_SIZE * 2) as f32) as i32
                                                                                 } else {
                                                                                    jt.y + (( (jt.orientation - PI/ 2f32) / 2f32 ).sin().abs() * (GRID_SIZE * 4) as f32) as i32
                                                                                 };
                                                                                 (_x, _y)

                                                                                 //(jt.x+(40 as f32 * jt.orientation.sin().abs()) as i32, 
                                                                                 // jt.y+(40 as f32 * jt.orientation.cos().abs()) as i32) 
                                                                               }, //TODO handle rotations
                                          _=>{ panic!("ASDF"); }  //TODO more informative panic
                                        };




                    let it_x2 = if it.orientation.sin().abs() < 0.001 { 
                                    it.x + ((it.orientation / 2f32).cos().abs() * (GRID_SIZE * 4) as f32) as i32 
                                } else { 
                                    it.x + (it.orientation.sin().abs() * (GRID_SIZE * 2) as f32) as i32
                                };

                    let it_y2 = if it.orientation.sin().abs() < 0.001 {
                                    it.y + (it.orientation.cos().abs() * (GRID_SIZE * 2) as f32) as i32
                                } else {
                                    it.y + (( (it.orientation - PI/ 2f32) / 2f32 ).cos().abs() * (GRID_SIZE * 4) as f32) as i32
                                };

                    //Depricated 02/08/2021
                    //let it_x2 = (it.orientation.cos().abs()*(GRID_SIZE * 4) as f32 ) as i32 + it_x1;
                    //let it_y2 = (it.orientation.sin().abs()*(GRID_SIZE * 4) as f32 ) as i32 + it_y1;


                    
                    let jt_x2 = if jt.orientation.sin().abs() < 0.001 { 
                                    jt.x + ((jt.orientation / 2f32).cos().abs() * (GRID_SIZE * 4) as f32) as i32 
                                } else { 
                                    jt.x + (jt.orientation.sin().abs() * (GRID_SIZE * 2) as f32) as i32
                                };

                    let jt_y2 = if jt.orientation.sin().abs() < 0.001 {
                                    jt.y + (jt.orientation.cos().abs() * (GRID_SIZE * 2) as f32) as i32
                                } else {
                                    jt.y + (( (jt.orientation - PI/ 2f32) / 2f32 ).cos().abs() * (GRID_SIZE * 4) as f32) as i32
                                };


                    //Depricated 02/08/2021
                    //let jt_x2 = (jt.orientation.cos().abs()*(GRID_SIZE * 4) as f32 ) as i32 + jt_x1;
                    //let jt_y2 = (jt.orientation.sin().abs()*(GRID_SIZE * 4) as f32 ) as i32 + jt_y1;



                    //if jt.circuit_element_type == SelectedCircuitElement::Battery {
                    //    println!("{} {}", i, j);
                    //    println!("battery {} {} {} {} {} {} {}", jt_x1, jt_y1, jt_x2, jt_y2, jt.x, jt.y, jt.orientation);
                    //    println!("it      {} {} {} {}", it_x1, it_y1, it_x2, it_y2);
                    //    println!("mouse {} {}", mouseinfo.x, mouseinfo.y);
                    //}

                    //if jt.circuit_element_type == SelectedCircuitElement::Resistor {
                    //    println!("{} {}", i, j);
                    //    println!("resistor {} {} {} {} {} {} {}", jt_x1, jt_y1, jt_x2, jt_y2, jt.x, jt.y, jt.orientation);
                    //    println!("it      {} {} {} {}", it_x1, it_y1, it_x2, it_y2);
                    //    println!("mouse {} {}", mouseinfo.x, mouseinfo.y);
                    //}


                    if (it_x1 == jt_x1 && it_y1 == jt_y1) 
                    || (it_x2 == jt_x2 && it_y2 == jt_y2) 
                    || (it_x2 == jt_x1 && it_y2 == jt_y1) 
                    || (it_x1 == jt_x2 && it_y1 == jt_y2) 
                    {
                        //TODO reset the current nodes it the nodes are not touched during this process
                        if (it_x1 == jt_x1 && it_y1 == jt_y1) {
                            app_storage.arr_circuit_elements[i].a_node  = it.a_node.min(jt.a_node);
                            app_storage.arr_circuit_elements[j].a_node  = it.a_node.min(jt.a_node);

                            for index in 0..it.a_index.len(){
                                match it.a_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[i].a_index[index] = Some(j);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if j == n { break; } },
                                }
                            }
                            for index in 0..jt.a_index.len(){
                                match jt.a_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[j].a_index[index] = Some(i);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if i == n { break; } },
                                }
                            }
                        } 
                        if (it_x2 == jt_x2 && it_y2 == jt_y2) {
                            app_storage.arr_circuit_elements[i].b_node  = it.b_node.min(jt.b_node);
                            app_storage.arr_circuit_elements[j].b_node  = it.b_node.min(jt.b_node);

                            //println!("1 {} {}", i, j);
                            for index in 0..it.b_index.len(){
                                match it.b_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[i].b_index[index] = Some(j);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if j == n { break; } },
                                }
                            }
                            for index in 0..jt.b_index.len(){
                                match jt.b_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[j].b_index[index] = Some(i);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if i == n { break; } },
                                }
                            }
                        }
                        if (it_x2 == jt_x1 && it_y2 == jt_y1) {
                            app_storage.arr_circuit_elements[i].b_node  = it.b_node.min(jt.a_node);
                            app_storage.arr_circuit_elements[j].a_node  = it.b_node.min(jt.a_node); 

                            for index in 0..it.b_index.len(){
                                match it.b_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[i].b_index[index] = Some(j);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if j == n { break; } },
                                }
                            }
                            for index in 0..jt.a_index.len(){
                                match jt.a_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[j].a_index[index] = Some(i);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if i == n { break; } },
                                }
                            }
                        }
                        if (it_x1 == jt_x2 && it_y1 == jt_y2) {
                            app_storage.arr_circuit_elements[i].a_node  = it.a_node.min(jt.b_node);
                            app_storage.arr_circuit_elements[j].b_node  = it.a_node.min(jt.b_node);

                            for index in 0..it.a_index.len(){
                                match it.a_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[i].a_index[index] = Some(j);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if j == n { break; } },
                                }
                            }
                            for index in 0..jt.b_index.len(){
                                match jt.b_index[index] {
                                    None=>{ app_storage.arr_circuit_elements[j].b_index[index] = Some(i);  break; }, //TODO some is incorrect
                                    Some(n)=>{ if i == n { break; } },
                                }
                            }
                        }
                    }
                }
            }



            
            let (mut c_matrix, pairs, hashmap) = compute_circuit(&mut app_storage.arr_circuit_elements);
            if pairs.len() == 0 {
                app_storage.run_circuit = false;
            }

            let mut bad_solved_value = false;
            for (i, it) in pairs.iter().enumerate(){

                let minus_one = c_matrix.columns - 1;
                let solved_current = *c_matrix.get_element(i, minus_one);
                let solved_voltage = *c_matrix.get_element(i+pairs.len(), minus_one);

                if bad_solved_value == false{
                    if !solved_current.is_finite() 
                    || !solved_voltage.is_finite(){
                        let text = format!("Error:  Current({}) and or voltage({}), is not finite. Impacts node {} {}.   Tip: Does the circuit include a resistor?", 
                                           solved_current, solved_voltage,
                                    app_storage.arr_circuit_elements[it.element_index].unique_a_node,
                                    app_storage.arr_circuit_elements[it.element_index].unique_b_node);
                        if !app_storage.messages.contains(&(MessageType::Error, text.clone())){
                            app_storage.messages.push((MessageType::Error, text));
                        }
                        bad_solved_value = true;
                        app_storage.run_circuit = false;
                    }
                }

                app_storage.arr_circuit_elements[it.element_index].solved_current = Some(solved_current);
                app_storage.arr_circuit_elements[it.element_index].solved_voltage = Some(solved_voltage);

                let mut print_current = solved_current;
                let mut print_voltage = solved_voltage;

                let mut _direction;

                if it.in_node == app_storage.arr_circuit_elements[it.element_index].a_node{


                    _direction = CircuitElementDirection::AtoB;
                    if solved_current < 0.0 {
                        app_storage.arr_circuit_elements[it.element_index].direction = Some( CircuitElementDirection::BtoA );
                    } else{
                        app_storage.arr_circuit_elements[it.element_index].direction = Some( CircuitElementDirection::AtoB );
                    }
                } else {

                    _direction = CircuitElementDirection::BtoA;
                    if solved_current < 0.0 {
                        app_storage.arr_circuit_elements[it.element_index].direction = Some( CircuitElementDirection::AtoB );
                    } else{
                        app_storage.arr_circuit_elements[it.element_index].direction = Some( CircuitElementDirection::BtoA );
                    }
                }


                //We correct current and voltage to match what we assume a convention. Where current is assumed to enter node A and exit node B.
                match app_storage.arr_circuit_elements[it.element_index].direction{
                    Some(CircuitElementDirection::AtoB)=>{
                        print_current = print_current.abs();
                    },
                    Some(CircuitElementDirection::BtoA)=>{
                        print_current = -1f32 * print_current.abs();
                    },
                    _=>{}
                }
                print_voltage = if print_current.signum() == -1f32 {  -1f32 * print_voltage.abs() } else { print_voltage.abs() };


                let element = app_storage.arr_circuit_elements[it.element_index];



                if element.circuit_element_type == SelectedCircuitElement::CustomVoltmeter{

                    let prev_voltage =  element.print_voltage.as_ref().unwrap_or(&0f32);
                    print_voltage = element.drift*prev_voltage + (1f32 - element.drift) * (print_voltage + element.bias) + sample_normal(element.noise);
                }
                if element.circuit_element_type == SelectedCircuitElement::CustomAmmeter{

                    let prev_current =  element.print_current.as_ref().unwrap_or(&0f32);
                    print_current = element.drift*prev_current + (1f32 - element.drift) * (print_current + element.bias) + sample_normal(element.noise);
                }


                app_storage.arr_circuit_elements[it.element_index].print_voltage = Some(print_voltage);
                app_storage.arr_circuit_elements[it.element_index].print_current = Some(print_current);
                let ce_time = app_storage.arr_circuit_elements[it.element_index].time;

                //Setting up current and graph storage
                let a_node = app_storage.arr_circuit_elements[it.element_index].unique_a_node;
                let b_node = app_storage.arr_circuit_elements[it.element_index].unique_b_node;
                match app_storage.saved_circuit_currents.get_mut(&(a_node, b_node)){
                    Some(currents_times)=>{
                        if currents_times[0].len() > MAX_SAVED_MEASURMENTS{
                            currents_times[0].remove(0);
                            currents_times[1].remove(0);
                        }

                        currents_times[0].push(print_current);
                        currents_times[1].push(ce_time);
                        //currents_times[1].push(app_storage.sim_time);

                    },
                    None=>{}
                }
                match app_storage.saved_circuit_volts.get_mut(&(a_node, b_node)){
                    Some(volts_times)=>{
                        if volts_times[0].len() > MAX_SAVED_MEASURMENTS{
                            volts_times[0].remove(0);
                            volts_times[1].remove(0);
                        }

                        let element = app_storage.arr_circuit_elements[it.element_index];
                        volts_times[0].push(print_voltage);
                        volts_times[1].push(ce_time);
                        //volts_times[1].push(app_storage.sim_time);

                    },
                    None=>{}
                }

            }


            //println!("# of elements: {}\n", app_storage.arr_circuit_elements.len());
            //for it in app_storage.arr_circuit_elements.iter(){
            //    println!("{:?}\n", it);
            //}
            //panic!();


            //NOTE this is to reset circuit elements
            for it in app_storage.arr_circuit_elements.iter_mut(){
                it.discovered = false;
                
                it.a_node = it.unique_a_node;
                it.b_node = it.unique_b_node;

                it.a_index = [None; 3];
                it.b_index = [None; 3];
            }
        }
    }


    if app_storage.teacher_mode {
        
        change_font(FONT_NOTOSANS_BOLD);

        draw_string(&mut os_package.window_canvas, "TA", -1, -1, C4_WHITE, 32.0);
        draw_string(&mut os_package.window_canvas, "TA", 0, 0, C4_RED, 32.0);

        change_font(FONT_NOTOSANS);
    }


    return 0;
}





pub enum PanelContent{
    Header(String),
    Text(String),
    Image(TGBitmap),
    Question(MultiQuestion),
}

pub struct Panel{
    pub contents: Vec<PanelContent>,
}


struct MultiQuestion{
    question: String,
    choices: Vec<String>,
    answer_index: Option<usize>,
    number_chances: usize,
    answers: Vec<usize>,
}


pub fn parse_and_panel_filebuffer(buffer: &str)->(Vec<Panel>, Vec<(MessageType, String)>){
    use std::fs::File;
    use std::io::prelude::*;

    let ( parsed_results, mut errors) = parse(buffer);
    let mut panels = vec![];

    for (i, it) in parsed_results.iter().enumerate(){

        let mut panel = Panel{contents: vec![]};

        let mut question_answer_index : Option<usize>= None;

        for jt in it.contents.iter() {
            match jt {
                Content::Text(string)=>{
                    panel.contents.push(PanelContent::Text(string.to_string()));
                },
                Content::Header(string)=>{
                    panel.contents.push(PanelContent::Header(string.to_string()));
                },
                Content::Image(string)=>{
                    //TODO should we get the image here?
                    //TODO strip preceeding and postceeding spaces

                    let mut temp_string = string.to_string();
                    if temp_string.chars().nth(0) == Some(' '){
                        temp_string.remove(0);
                    }
                  
                    if temp_string.chars().nth(temp_string.len()) == Some(' '){
                        let l = temp_string.len()-1;
                        temp_string.remove(l);
                    }
                    
                    match File::open(temp_string){
                        Ok(mut f)=>{
                            let mut buffer = Vec::new();
                            f.read_to_end(&mut buffer).expect("Buffer could not be read.");
                            let image = stbi_load_from_memory_32bit(&buffer);
                            
                            match image {
                                Ok(im)=>{ 
                                    let mut bmp = TGBitmap{info_header: TGBitmapHeaderInfo{
                                                            header_size:        0u32,
                                                            width:              im.width,
                                                            height:             im.height,
                                                            planes:             1,
                                                            bit_per_pixel:      32,
                                                            compression:        0u32,
                                                            image_size:         0u32,
                                                            x_px_per_meter:     0i32,
                                                            y_px_per_meter:     0i32,
                                                            colors_used:        0u32,
                                                            colors_important:   0u32,
                                                      },
                                                       file_header: TGBitmapFileHeader::default(),
                                                       rgba: Vec::with_capacity(im.buffer.len()),
                                                       //rgba: im.buffer,
                                                       width: im.width,
                                                       height: im.height};//TODO I don't like the bit map struct
                                    for _j in 1..(im.height+1) as usize{
                                        let j =  (im.height as usize - _j) * im.width as usize;

                                        for _i in 0..im.width as usize{
                                            let i =  _i;
                                            let k = (i+j)*4;

                                            bmp.rgba.push( im.buffer[k+2] );
                                            bmp.rgba.push( im.buffer[k+1] );
                                            bmp.rgba.push( im.buffer[k+0] );
                                            bmp.rgba.push( im.buffer[k+3] );
                                        }
                                    }
                                    panel.contents.push(PanelContent::Image(bmp)); 
                                },
                                _=>{
                                    //TODO should we do something here?
                                    errors.push(( MessageType::Error, format!("Error:  Image, {}, on panel {}, could not be opened by program.", string, i)) );
                                }
                            }
                        },
                        Err(_)=>{
                        },
                    }
                },
                Content::Question(string)=>{
                    let mut question_and_answers = MultiQuestion::new();
                    question_and_answers.question = string.to_string();

                    panel.contents.push(PanelContent::Question(question_and_answers));
                    question_answer_index = Some(panel.contents.len()-1);
                },
                Content::Answer(string, bool_)=>{
                    match question_answer_index{
                        Some(qa_idex)=> {
                            match &mut panel.contents[qa_idex]{
                                PanelContent::Question(qa)=>{
                                    qa.choices.push(string.to_string());
                                    let index = qa.choices.len() - 1;
                                    if *bool_ {
                                        qa.answer_index = Some(index);
                                    }
                                },
                                _=>{panic!("we did something wrong with quetions.")}
                            }
                        },
                        None=>{}
                    }
                },
                _=>{panic!();}
            }
        }
        for it in panel.contents.iter(){
            match it { 
                PanelContent::Question(qa)=>{
                    if qa.question.len() > 0 
                    && qa.answer_index.is_none(){  
                        errors.push(( MessageType::Error, format!("Error:  Question in panel({}), has no correct answer.", 
                        i)));
                    } 
                },
            _=>{}
            } 
        } 

        panels.push(panel);
    }
    return (panels, errors);
}










impl MultiQuestion{
    pub fn new()->MultiQuestion{ //TODO currently we only allow for one  correct answer do we want to keep this
        MultiQuestion{
            question: String::new(),
            choices: Vec::new(),
            answer_index: None,
            number_chances: 3,
            answers: Vec::new(),
        }
    }
}

//TODO move to misc tools or rendertools
pub fn generate_wrapped_strings(string: &str, font: f32, max_width: i32)->Vec<String>{
    let mut strings= vec![String::new()];
    let mut strings_index = 0;

    for jt in string.split('\n'){
    for it in jt.split(' '){
        if get_advance_string(&strings[strings_index], font) + get_advance_string(it, font) < max_width - font as i32 {
        } else {
            strings_index += 1;
            strings.push(String::new());
        }
        strings[strings_index].push_str(it);
        strings[strings_index].push_str(" ");
    }
        strings_index += 1;
        strings.push(String::new());
    }
    return strings;
}


//TODO
//this is hella slow
//move to render tools
//we have artifacts we should correct for
fn rotate_bmp(src_bmp: &mut TGBitmap, angle: f32, inplace: bool)->Option<TGBitmap>{unsafe{

    fn calc_xy(x: f32, y: f32, angle: f32)->(f32, f32){
        let cos = angle.cos();
        let sin = angle.sin();
        let rt = ( (x as f32 * cos + y as f32 * sin).round(),
                   (-x as f32 * sin + y as f32 * cos).round()
                 );
        return rt;
    }


    if inplace {
        panic!("TODO: Rotate bmp inplace is on the todo list.");
    } else {
        let dst_w = (src_bmp.width as f32 * angle.cos().abs() + src_bmp.height as f32 * angle.sin().abs()).round() as isize;
        let dst_h = (src_bmp.width as f32 * angle.sin().abs() + src_bmp.height as f32 * angle.cos().abs()).round() as isize;
        let mut dst_bmp = TGBitmap::new(dst_w as i32, dst_h as i32);//TODO


        let src_buffer = src_bmp.rgba.as_ptr();
        let dst_buffer = dst_bmp.rgba.as_mut_ptr();

        let w = src_bmp.width as isize;
        let h = src_bmp.height as isize;

        for j in 0..dst_h{
            for i in 0..dst_w{
                let dst_offset = 4*(i + j*dst_w) as isize;
                let __x = i as f32 - w as f32/2f32;
                let __y = j as f32 - h as f32/2f32;

                let (mut x, mut y) = calc_xy( __x, __y, angle);
                x += w as f32 / 2f32;
                y += h as f32 / 2f32;

                let x = x.round() as isize;
                let y = y.round() as isize;


                if x > w as isize || x < 0 { 
                    continue; }
                if y > h as isize || y < 0 { 
                    continue; }
                if x+y*w >= h*w {
                    continue;
                }

                let src_offset=4*(x+y*w as isize);

                let a = *src_buffer.offset(src_offset + 3);
                let r = *src_buffer.offset(src_offset + 2);
                let g = *src_buffer.offset(src_offset + 1);
                let b = *src_buffer.offset(src_offset + 0);

                *dst_buffer.offset(dst_offset + 3) = a;
                *dst_buffer.offset(dst_offset + 2) = r;
                *dst_buffer.offset(dst_offset + 1) = g;
                *dst_buffer.offset(dst_offset + 0) = b;
            }
        }

        return Some(dst_bmp);
    }
    return None;
}}







//TODO this is going to need alot of work
mod parser{

use crate::lab_sims::MessageType;

const example : &str =
"//comments should be ignored
#Section 
#Header Beginning
#Text
This is the beginning of the lab.
The lab is about a great many things that you will never understand.
Sorry.

#Section 
#Header Initial Questions
#Text
Lets begin with some questions.
Please do you best answers will be graded harshly.

#Question
How large is the farm?

#AnswerWrong   Pint sized.
#AnswerCorrect Byte sized.
#AnswerWrong   bit sized.
#AnswerWrong   bite sized.

#Section
#Text
I hope you got that anwser right.
LOL
#image screenshot.bmp

";


    fn peek(i: usize, buffer: &[char])->Option<char>{
        //TODO
        if i+1 >= buffer.len(){
            return None;
        } else {
            return Some(buffer[i+1]);
        } 
        return None;        
    }

    fn peek_is_char(i: usize, buffer: &[char], character: char)->bool{
        let c = peek(i, buffer);
        match c {
            Some(_c)=>{
                if _c == character { return true; }
            },
            _=>{return false;}
        }
        return false;
    }
    
    #[derive(Debug)]
    pub struct Section{
        pub contents: Vec<Content>,
    }
    #[derive(Debug)]
    pub enum Content{
      Question(String),
      Header(String),
      Text(String),
      Answer(String, bool),
      Image(String),
    }


    enum Mode{
    }
    pub fn parse(input: &str)->(Vec<Section>, Vec<(MessageType, String)>){//TODO maybe return errors?
        use Content::*;
        let mut char_arr : Vec<char> = input.chars().collect();

        //NOTE strip comments
        {
            let mut strip_i = 0;
            loop{
                let _char = char_arr[strip_i];
                strip_i += 1;
                if strip_i >= char_arr.len(){
                    
                    break;
                }


                if _char == '/'
                && char_arr[strip_i] == '/'{
                    let mut j = 0;
                    if strip_i >= char_arr.len(){
                        break;
                    }
                    '_b: loop{
                        if peek_is_char(strip_i + j, &char_arr, '\n'){
                            break '_b;
                        }
                        j += 1;
                    }
                    for _remove in (strip_i-1..strip_i+j+1).rev(){
                        char_arr.remove(_remove);
                    }
                }
            }
        }
        let mut rt = vec![];
        let mut errs = vec![];
        let mut i = 0;

        'a: loop{

            if char_arr.len() == 0{
                break 'a;
            }
            if i >= char_arr.len(){
                break 'a;
            }
            let _char = char_arr[i];
           
            let mut token_buffer = "".to_string();

 
            if _char == '/' 
            && peek_is_char(i, &char_arr, '/'){
                let mut j = 0;
                'b: loop{

                    if i + j >= char_arr.len(){
                        
                        break 'a;
                    }

                    if peek_is_char(i+j, &char_arr, '\n') {
                        i += j;
                        break 'b;
                    }
                    j += 1;
                }
            } else if _char == '\n' 
            && peek_is_char(i, &char_arr, '#'){
                //NOTE update token_buffer

                let mut j = 2;
                if i+j < char_arr.len() {

                    'c: loop{
                        if i + j >= char_arr.len(){
                            break 'a;
                        }

                        token_buffer.push(char_arr[i+j]);

                        if peek_is_char(i+j, &char_arr, ' ') 
                        || peek_is_char(i+j, &char_arr, '\n'){
                            i += j+1;
                            break 'c;
                        }
                        j += 1;
                    }
                }
            } else {
                let mut token_buffer = "text".to_string();
            }

            
            match token_buffer.to_lowercase().as_str(){
                "section"=>{
                    rt.push(Section{contents: Vec::new()});
                },
                "header" | "text" | "question" =>{
                    if rt.len() == 0{
                        //TODO we need to throw a warning
                        i+=1;
                        continue 'a;
                    }
                    let mut j = 0;
                    let mut string = String::new();

                    'd: loop{
                        if i + j >= char_arr.len(){
                            let rt_index = rt.len() - 1;
                            match token_buffer.to_lowercase().as_str(){
                                "header"=>{ rt[rt_index].contents.push(Header(string)); }
                                "text"=>{ rt[rt_index].contents.push(Text(string)); }
                                "question"=>{ rt[rt_index].contents.push(Question(string)); }
                                 _=>{ }
                            }
                            break 'a;
                        }

                        if char_arr[i+j] == '\n' 
                        && j == 0 {
                            j+=1;
                            continue 'd;
                        }
                        if char_arr[i+j] == ' ' 
                        && j == 0 {
                            j+=1;
                            continue 'd;
                        }

                        if char_arr[i+j] == '\n'
                        && peek_is_char( i+j, &char_arr, '#'){
                            i += j;
                            break 'd;
                        }

                        string.push(char_arr[i+j]);
                        j += 1;
                    }
                    let rt_index = rt.len() - 1;
                    
                    match token_buffer.to_lowercase().as_str(){
                        "header"=>{ rt[rt_index].contents.push(Header(string)); }
                        "text"=>{ rt[rt_index].contents.push(Text(string)); }
                        "question"=>{ rt[rt_index].contents.push(Question(string)); }
                         _=>{ }
                    }
                },
                "image"=>{
                    if rt.len() == 0{
                        //TODO we need to throw a warning
                        break 'a;
                    }
                    let mut j = 0;
                    let mut string = String::new();

                    'e: loop{
                        if i + j >= char_arr.len(){
                            let rt_index = rt.len() - 1;

                            if string.chars().nth(0) == Some(' '){
                                string.remove(0);
                            }
                          
                            if string.chars().nth(string.len()) == Some(' '){
                                let l = string.len()-1;
                                string.remove(l);
                            }

                            if !std::path::Path::new(&string).exists()  
                            { 
                                errs.push(( MessageType::Error, format!("Error:  {} does not exist", string)));
                            }

                            rt[rt_index].contents.push(Image(string)); //TODO we could path this here?
                            break 'a;
                        }
                        if char_arr[i+j] == '\n' 
                        && j == 0 {
                            j+=1;
                            continue 'e;
                        }
                        if char_arr[i+j] == '\n'
                        && j > 0{
                            i += j;
                            break 'e;
                        }

                        string.push(char_arr[i+j]);
                        j += 1;
                    }
                    let rt_index = rt.len() - 1;


                    if string.chars().nth(0) == Some(' '){
                        string.remove(0);
                    }
                  
                    if string.chars().nth(string.len()) == Some(' '){
                        let l = string.len()-1;
                        string.remove(l);
                    }

                    if !std::path::Path::new(&string).exists()  
                    { errs.push( ( MessageType::Error,  format!("Error:  Image file, {}, does not exist.", string)))}
                    rt[rt_index].contents.push(Image(string)); 
                },
                "answercorrect" | "answerwrong"=>{
                    //TODO
                    if rt.len() == 0{
                        //TODO we need to throw a warning
                        break 'a;
                    }
                    let mut j = 0;
                    let mut string = String::new();
                    'f: loop{
                        if i + j >= char_arr.len(){
                            let rt_index = rt.len() - 1;
                            match token_buffer.to_lowercase().as_str(){
                                "answercorrect"=>{ rt[rt_index].contents.push(Answer(string, true)); }
                                "answerwrong"=>{ rt[rt_index].contents.push(Answer(string, false)); }
                                 _=>{ }
                            }
                            break 'a;
                        }
                        if char_arr[i+j] == '\n' 
                        && j == 0 {
                            j+=1;
                            i += j;
                            continue 'f;
                        }
                        if char_arr[i+j] == '\n'
                        && peek_is_char( i+j, &char_arr, '#'){
                            i += j;
                            break 'f;
                        }

                        string.push(char_arr[i+j]);
                        j += 1;
                    }
                    let rt_index = rt.len() - 1;
                    match token_buffer.to_lowercase().as_str(){
                        "answercorrect"=>{ rt[rt_index].contents.push(Answer(string, true)); }
                        "answerwrong"=>{ rt[rt_index].contents.push(Answer(string, false)); }
                         _=>{ }
                    }
                    
                },
                _=>{ i+= 1; }
            }
            token_buffer.clear();//TODO
        }


        return (rt, errs);
    }


}








pub fn draw_grid(canvas: &mut WindowCanvas, grid_size: i32){
    let canvas_w = canvas.w;
    let canvas_h = canvas.h;

    for i in 0..canvas_w/grid_size{
        let x = i * grid_size - 1;
        draw_rect(canvas, [x, 0, 2, canvas_h], COLOR_GRID, true);
    }
    for i in 0..canvas_h/grid_size{
        let y = i * grid_size - 1;
        draw_rect(canvas, [0, y, canvas_w, 2], COLOR_GRID, true);
    }
}





mod matrixmath{
    pub struct Matrix{
        pub arr : Vec<f32>,
        pub rows : usize,
        pub columns : usize,
    }
    impl Matrix{
        pub fn zeros(rows: usize, columns: usize)->Matrix{
            if rows == std::usize::MAX 
            || columns == std::usize::MAX{
                panic!("Exceeded size limits.");
            }
            Matrix{
                arr: vec![0.0f32; rows*columns],
                rows : rows,
                columns : columns,
            }
        }
        pub fn identity(rows_columns: usize)->Matrix{
            if rows_columns == std::usize::MAX {
                panic!("Exceeded size limits.");
            }
            let mut m = Matrix::zeros(rows_columns, rows_columns);
            for i in 0..rows_columns{
                *m.get_element(i, i) = 1.0;
            }
            return m;
        }
        pub fn grow(&mut self, add_n_rows: usize, add_n_columns: usize){

            let mut v = Matrix::zeros(self.rows+add_n_rows, self.columns+add_n_columns);
            for r in 0..self.rows{
                for c in 0..self.columns{
                    *v.get_element(r, c) = *self.get_element(r, c);
                }
            }
            self.rows += add_n_rows;
            self.columns += add_n_columns;
            self.arr = v.arr;
            
        }

        //TODO might be nice to incorportate numpy style -1 symantics
        pub fn get_element(&mut self, row: usize, column: usize)->&mut f32{
            return &mut self.arr[column + self.columns * row];
        }

        pub fn multiply(&self, matrix: &Matrix)->Matrix{
            if self.columns != matrix.rows
            || self.rows  != matrix.columns
            { panic!("Column Row mismatch: {} {} ||  {} {}", self.columns, self.rows, matrix.columns, matrix.rows); }

            let mut rt_matrix = Matrix::zeros(self.rows, matrix.columns);

            for j in 0..rt_matrix.rows{
                for i in 0..rt_matrix.columns{
                    let value = {
                        let mut temp = 0.0;
                        for k in 0..self.columns{
                            temp += self.arr[k + j*self.columns] * matrix.arr[ i + k*matrix.columns];
                        }
                        temp
                    };
                    rt_matrix.arr[ i + rt_matrix.columns*j ] = value;
                }
            }
            return rt_matrix;
        }
        pub fn transpose(&self)->Matrix{
            let mut rt_matrix = Matrix::zeros(self.columns, self.rows);
            for j in 0..rt_matrix.rows{
                for i in 0..rt_matrix.columns{
                    rt_matrix.arr[ i + rt_matrix.columns*j ] = self.arr[j + self.columns*i];

                }
            }
            return rt_matrix;
        }


/////////////////////////////////////////////
/* https://en.wikipedia.org/wiki/Gaussian_elimination

h := 1 /* Initialization of the pivot row */
k := 1 /* Initialization of the pivot column */

while h ≤ m and k ≤ n
    /* Find the k-th pivot: */
    i_max := argmax (i = h ... m, abs(A[i, k]))
    if A[i_max, k] = 0
        /* No pivot in this column, pass to next column */
        k := k+1
    else
         swap rows(h, i_max)
         /* Do for all rows below pivot: */
         for i = h + 1 ... m:
                f := A[i, k] / A[h, k]
                /* Fill with zeros the lower part of pivot column: */
                A[i, k] := 0
                /* Do for all remaining elements in current row: */
                for j = k + 1 ... n:
                     A[i, j] := A[i, j] - A[h, j] * f
         /* Increase pivot row and column */
         h := h + 1
         k := k + 1
*/
        pub fn gaussian_elimination(&mut self){
            fn swap_rows(m1: &mut Matrix, row1: usize, row2: usize){
                for i in 0..m1.columns{

                    let temp = m1.arr[i + m1.columns*row1];
                    m1.arr[i + m1.columns*row1] = m1.arr[i+m1.columns*row2];
                    m1.arr[i + m1.columns*row2] = temp;
                }
            }
            let mut h = 0;
            let mut k = 0;

            let m = self.rows;
            let n = self.columns;

            while h < m && k < n {
                let i_max : usize = {
                    let mut rt = 0;
                    let mut f = 0.0;
                    for i in h..m {
                        if self.arr[k + i*self.columns].abs() > f { f = self.arr[k + i*self.columns].abs(); rt = i; }
                    }
                    rt
                };
                if self.arr[k + i_max*self.columns] == 0.0 { k+=1; continue; }
                else{
                    swap_rows(self, h, i_max);

                    for i in h+1..m{
                        let f = self.arr[k + i*self.columns] / self.arr[k + h*self.columns];
                        self.arr[k + i*self.columns] = 0.0;
                        for j in k+1..n{
                             self.arr[j + i*self.columns] = self.arr[j + i*self.columns] - self.arr[j + h*self.columns] * f;
                        }
                    }
                    h += 1;
                    k += 1;
                }
            }

            ///////////////////////////////
            //TODO NEW
            //remove offdiagonals
            let mut minus_i = self.columns-1;
            for i in (0..self.rows).rev(){

                minus_i -= 1;
                let minus_1 = self.columns - 1;
                self.arr[i*self.columns+minus_1] = self.arr[i*self.columns+minus_1] / self.arr[i*self.columns+minus_i];
                self.arr[i*self.columns+minus_i] = 1.0;

                for j in 0..i{
                    self.arr[j*self.columns+minus_i] *= self.arr[i*self.columns+minus_1];
                    self.arr[j*self.columns+minus_1] -= self.arr[j*self.columns+minus_i];
                    self.arr[j*self.columns+minus_i] = 0.0;
                }
            }
            ///////////////////////
        }

    }
  impl PartialEq for Matrix{
      fn eq(&self, x: &Self)->bool{
          if self.rows == x.rows
          && self.columns == x.columns {
              for i in 0..self.arr.len(){
                  if (self.arr[i] - x.arr[i]).abs() > 0.0001 { return false; }
              }
          } else {
              return false;
          }
          return true;
      }
  }
use std::fmt;
  impl fmt::Debug for Matrix{
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{

          let mut array_string = format!("Matrix rows {}, columns {} \n", self.rows, self.columns);
          array_string.push('[' );
          for (i, it) in self.arr.iter().enumerate(){
              array_string.push_str( &it.to_string() );
              array_string.push_str( ", " );
              if (i+1) %  self.columns == 0 && i != self.arr.len() - 1{
              array_string.push( '\n' );
              }
          }
          array_string.push( ']' );


          f.write_str(&array_string)
      }
    /*fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }*/
  }

#[test]
fn gaussianelimination_test(){
    let mut m1 = Matrix{
        arr: vec![ 2.0,  1.0, -1.0, 8.0,
                  -3.0, -1.0,  2.0, -11.0,
                  -2.0,  1.0,  2.0, -3.0,
                 ],
        rows: 3,
        columns: 4,
    };
    let mut m2 = Matrix{
        arr: vec![ 1.0,  0.0,  0.0, 2.0,
                   0.0,  1.0,  0.0, 3.0,
                   0.0,  0.0,  1.0, -1.0,
                 ],
        rows: 3,
        columns: 4,
    };

    m1.gaussian_elimination();
    assert_eq!(m2, m1);

    let mut m3 = Matrix{
        arr: vec![
            1.0, 1.0, 3.0,
            3.0, -2.0, 4.0,
        ],
        rows: 2,
        columns: 3,
    };
    let mut m4 = Matrix{
        arr:vec![
            1.0, 0.0, 2.0,
            0.0, 1.0, 1.0,
        ],
        rows: 2,
        columns: 3,
    };

    m3.gaussian_elimination();
    assert_eq!(m4, m3);

    let mut m1 = Matrix{
        arr: vec![
                    -1.0,  1.0, 0.0, 0.0,  0.0, 0.0,
                     0.0,  0.0, 1.0, 0.0,  1.0, 0.0,
                     0.0,  0.0, 0.0, 1.0, -1.0, 0.0,
                     0.0,  0.0, 1.0, 0.0,  0.0, 2.0,
                     0.0, -0.5, 0.0, 1.0,  0.0, 0.0,
        ],
        rows: 5,
        columns: 6,
    };

    m1.gaussian_elimination();
    
    //let mut m = Matrix{ rows: 7, columns: 11,
    //     arr: vec![1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, 0.0, 
    //              -1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, 0.0, 
    //               0.0,  0.0,  0.0,  0.0,  0.0, -1.0,  0.0,  0.0,  1.0,  1.0, 0.0, 
    //               0.0, -1.0,  0.0,  0.0, -1.0,  1.0, -1.0,  0.0,  0.0,  0.0, 0.0, 
    //               0.0,  0.0, -1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, 0.0, 
    //               0.0,  0.0,  0.0, -1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0, 0.0, 
    //               0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  1.0, -1.0,  0.0, 0.0, 
    //               0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  1.0, -1.0,  0.0,  0.0, 0.0, ],
    //};
    //m.gaussian_elimination();
    //panic!("{:?}", m);

}

#[test]
fn transpose_test(){
    let m1 = Matrix{
        arr: vec![1.0, 2.0],
        rows: 1,
        columns: 2,
    };
    let m2 = Matrix{
        arr: vec![1.0, 2.0],
        rows: 2,
        columns: 1,
    };

    assert_eq!(m1.transpose(), m2);

    let m1 = Matrix{
        arr: vec![1.0, 2.0,
                  3.0, 4.0,
                 ],
        rows: 2,
        columns: 2,
    };

    let m2 = Matrix{
        arr: vec![1.0, 3.0,
                  2.0, 4.0,
                 ],
        rows: 2,
        columns: 2,
    };
    assert_eq!(m1.transpose(), m2);

    let m1 = Matrix{
        arr: vec![1.0, 2.0,
                  3.0, 4.0,
                  5.0, 6.0,
                 ],
        rows: 3,
        columns: 2,
    };

    let m2 = Matrix{
        arr: vec![1.0, 3.0, 5.0,
                  2.0, 4.0, 6.0,
                 ],
        rows: 2,
        columns: 3,
    };
    assert_eq!(m1.transpose(), m2);

}

#[test]
fn mult_test(){
    let m1 = Matrix{
        arr: vec![1.0, 2.0, 3.0],
        rows:1,
        columns:3,
    };
    let m2 = Matrix{
        arr: vec![4.0, 5.0, 6.0],
        rows:3,
        columns:1,
    };

    let m3 = Matrix{
        arr: vec![32.0],
        rows: 1,
        columns: 1,
    };

    let m4 = Matrix{
        arr: vec![4.0, 8.0,  12.0,
                  5.0, 10.0, 15.0,
                  6.0, 12.0, 18.0],
        rows: 3,
        columns: 3,
    };

    assert_eq!(m1.multiply(&m2), m3);
    assert_eq!(m2.multiply(&m1), m4);
    let m5 = Matrix{
        arr: vec![1.0, 2.0,
                  3.0, 4.0,],
        rows: 2,
        columns: 2,
    };
    let m6 = Matrix{
        arr: vec![2.0, 0.0,
                  1.0, 2.0,],
        rows: 2,
        columns: 2,
    };
    let m7 = Matrix{
        arr: vec![4.0, 4.0,
                  10.0, 8.0,],
        rows: 2,
        columns: 2,
    };
    let m8 = Matrix{
        arr: vec![2.0, 4.0,
                  7.0, 10.0,],
        rows: 2,
        columns: 2,
    };
    assert_eq!(m5.multiply(&m6), m7);
    assert_eq!(m6.multiply(&m5), m8);

}

}


/*
1  procedure BFS(G, root) is
2      let Q be a queue
3      label root as discovered
4      Q.enqueue(root)
5      while Q is not empty do
6          v := Q.dequeue()
7          if v is the goal then
8              return v
9          for all edges from v to w in G.adjacentEdges(v) do
10             if w is not labeled as discovered then
11                 label w as discovered
12                 w.parent := v
13                 Q.enqueue(w)
*/
#[derive(Copy, Clone, Debug, PartialEq)]
struct Pair{ 
    in_node: usize, 
    out_node: usize, 
    good: bool, 
    element_index: usize
}
fn compute_circuit(graph : &mut Vec<CircuitElement>)->(Matrix, Vec<Pair>, Vec<usize>){
    
    let mut rt = Matrix::zeros(0,0);
    let mut queue = vec![0]; 
    if graph[0].discovered{ //TODO not sure what this is about
        return (rt, vec![], Vec::new());
    }

    let mut unique_nodes = vec![];
    for it in graph.iter(){ 
        if !unique_nodes.contains(&it.a_node){
            unique_nodes.push(it.a_node)
        }
        if !unique_nodes.contains(&it.b_node){
            unique_nodes.push(it.b_node)
        }
    }
    //Prune nodes
    //TODO we should while loop this just in case there 
    //more nodes to be pruned after first removal

    let mut element_was_removed = true;
    while element_was_removed == true{
        element_was_removed = false;

        let mut remove = vec![];
        for (j, jt) in unique_nodes.iter().enumerate(){ 
            let mut n = 0;
            for it in graph.iter(){ 
                if it.b_node == *jt
                || it.a_node == *jt{
                    n+=1;
                }
            }
            if n < 2{
                remove.push(j);
                element_was_removed = true;
            }
        }
        for it in remove.iter().rev(){
            unique_nodes.remove(*it);
        }
    }
   
//println!("unique nodes: {:?}", &unique_nodes);




    fn is_pair_good(current_pair_index: usize, pairs: &mut Vec<Pair>, exit_node: usize)->bool{
        //TODO 
        //This function attempts to trace current_pair back to the exit_node. If it cannot the pair is bad.
        //By definition the original function should handle this. We will keep it because it might be useful in the future.
        let current_pair = pairs[current_pair_index];
        if current_pair.in_node == exit_node{
            pairs[current_pair_index].good = true;
            return true;
        } 
        for i in 0..pairs.len(){
            
            if current_pair.in_node == pairs[i].out_node{
                
                let b = is_pair_good(i, pairs, exit_node);
                pairs[current_pair_index].good |= b;
                return b;
            }
        }
        return false;
    }





    let mut graph_index = 0;
    let mut graph_a_node = 0;
    let mut graph_b_node = 0;

    'a: loop{//NOTE checking for non first element
        let a_node = graph[graph_index].a_node;
        let b_node = graph[graph_index].b_node;

        let mut found_a = false;
        let mut found_b = false;

        for (i, it) in graph.iter().enumerate() { //TODO we should check both nodes. It there in an and out going connects the element is most likely good.

            if i != graph_index{
                if a_node == it.a_node 
                || a_node == it.b_node {
                    found_a = true;
                }

                if b_node == it.b_node
                || b_node == it.a_node{
                    found_b = true;
                }
            }
        }

        if found_a 
        && found_b {
            graph_a_node = a_node;
            graph_b_node = b_node;
            break 'a;
        }
        graph_index += 1;
        if graph_index >= graph.len(){
            return (rt, vec![], Vec::new());
        }
    }

    let mut init_index = 0;
    let mut found_index = false;
    for (i, it) in unique_nodes.iter().enumerate(){
        if graph_a_node == *it {
            found_index = true;
            init_index = i;
        }
    }
    if found_index == false{
        panic!("could not find index.");
    }

    let exit_node = unique_nodes[init_index]; 

    let mut queue = vec![unique_nodes[init_index]]; //TODO we are making an assumption here
    let mut pairs = vec![];
    let mut init = true;

    let mut prev_pair = Pair{in_node: 0, out_node: 0, good: false, element_index: 0};

    
    while queue.len() > 0 {
        let node = queue.pop().unwrap();

//TODO If setup has not been initialized should and the 
//first graph node has no connecter perhaps we should
//skip to the next node in the unique node list.
        for (i, it) in graph.iter().enumerate(){
            if node == it.a_node || node == it.b_node{
                if !unique_nodes.contains(&it.a_node)
                || !unique_nodes.contains(&it.b_node){
                    continue;
                }

                let mut next_node;

                if node == it.a_node {
                    next_node = it.b_node;
                } else {
                    next_node = it.a_node;
                }
                let mut pair = Pair{in_node: node, out_node: next_node, good: false, element_index: i};
                if init {

                    queue.push(next_node);
                    pairs.push(pair);
                    prev_pair = pair;


                    init = false; //Removed because there could multiple elements connected to the initial node
                    //break;//TODO This break might be causing issues
                }


                //TODO the following was done because I am lazy. I should not need to check each state this way.
                if pairs.contains(&Pair{in_node: pair.in_node, out_node: pair.out_node, good: false, element_index: pair.element_index}) {
                } else if pairs.contains( &Pair{ in_node: pair.out_node, out_node: pair.in_node, good: false, element_index: pair.element_index} ){
                } else if pairs.contains(&Pair{in_node: pair.out_node, out_node: pair.in_node, good: true, element_index: pair.element_index}) { 
                } else if pairs.contains(&Pair{in_node: pair.in_node, out_node: pair.out_node, good: true, element_index: pair.element_index}) { 

                    is_pair_good(pairs.len() - 1, &mut pairs, exit_node);
                } else {

                    //println!("q: {:?}", &queue);
                    //println!("pairs: {:?}", &pairs);
                    if next_node == exit_node{
                        
                        pairs.push(pair);
                        is_pair_good(pairs.len() - 1, &mut pairs, exit_node);
                        let i_pairs = pairs.len()-1;
                        pairs[i_pairs].good = true;


                        break;//TODO Prob should not break
                    } else {
                        queue.push(next_node);
                        pairs.push(pair);
                        prev_pair = pair;

                    }
                }
            } else {}
        }
    }

//    {//Prune pairs
//        let mut bad_nodes = vec![];
//        for (j, jt) in pairs.iter().enumerate(){
//            if jt.good == false {
//                bad_nodes.push(j);
//            }
//        }
//        for it in bad_nodes.iter().rev(){
//println!("remove {:?} {}", pairs[*it], *it);
//            pairs.remove(*it);
//        }
//    }

    {//NOTE prune unique_nodes
        let mut bad_nodes = vec![];
        for (i, it) in unique_nodes.iter().enumerate(){
            let mut is_node_good = false;
            for jt in pairs.iter(){
                if jt.in_node == *it
                || jt.out_node == *it{
                    is_node_good = true;
                    break;
                }
            }
            if !is_node_good { 
                bad_nodes.push(i);
            }
        }
        for it in bad_nodes.iter().rev(){
            unique_nodes.remove(*it);
        }
    }
//println!("DEF {:?}", &pairs); 
//println!("DEF {}", pairs.len()); 
////TODO
    //let mut pairs = vec![]
    //for it in unique_nodes.iter(){

    //}
    


    {//NOTE Set correct circuit element direction
        let mut m = Matrix::zeros(unique_nodes.len(), pairs.len()); 
        for (i, it) in pairs.iter().enumerate(){
            let in_node  = match unique_nodes.binary_search(&it.in_node) {
                Ok(n) => {n},
                Err(_) => { 
                    println!("There was no in_node found in unique. {:?} {} {}", it, graph[it.element_index].unique_a_node, graph[it.element_index].unique_b_node);
                    return (rt, vec![], Vec::new());
                },
            };
            let out_node = match unique_nodes.binary_search(&it.out_node){
                Ok(n) => {n},
                Err(_) => { 
                    println!("There was no out_node found in unique. {:?} {} {} | {} {}", it, graph[it.element_index].unique_a_node, graph[it.element_index].unique_b_node, 
                                                                                               graph[it.element_index].a_node, graph[it.element_index].b_node);
                    return (rt, vec![], Vec::new());
                },
            };

            *m.get_element(in_node, i)  = 1.0;
            *m.get_element(out_node, i) = 1.0;
        }
       
        undirected_to_directed_matrix(&mut m);
    
        
        for (i, it) in pairs.iter_mut().enumerate(){
            let in_node  = match unique_nodes.binary_search(&it.in_node) {
                Ok(n) => {n},
                Err(_) => { 
                    println!("There was no in_node found in unique. {:?} {} {}", it, graph[it.element_index].unique_a_node, graph[it.element_index].unique_b_node);
                    return (rt, vec![], Vec::new());
                },
            };
            let out_node = match unique_nodes.binary_search(&it.out_node){
                Ok(n) => {n},
                Err(_) => { 
                    println!("There was no out_node found in unique. {:?} {} {} | {} {}", it, graph[it.element_index].unique_a_node, graph[it.element_index].unique_b_node, 
                                                                                               graph[it.element_index].a_node, graph[it.element_index].b_node);
                    return (rt, vec![], Vec::new());
                },
            };

            let temp_in = it.in_node;
            let temp_out = it.out_node;

            if *m.get_element(in_node, i)  == -1.0{
                it.in_node = temp_out;
            }

            if *m.get_element(out_node, i) == 1.0{
                it.out_node = temp_in;
            }
            let v_in = *m.get_element(in_node, i); 
            let v_out = *m.get_element(out_node, i);

        }
    }



//println!("Pairs:");
//    for i in 0..pairs.len(){
//        if !pairs[i].good { 
//            is_pair_good(i, &mut pairs, exit_node);
//        }
//        println!("{:?}", pairs[i]);
//    }

//println!("GOOD Pairs:");
//    for i in 0..good_pairs.len(){
//        println!("{:?}", good_pairs[i]);
//    }

    


    let mut m = Matrix::zeros(unique_nodes.len(), pairs.len()); 
    let mut z = Matrix::zeros(pairs.len(), pairs.len());
    let mut y = Matrix::identity(pairs.len()); 

    let mut s = Matrix::zeros(pairs.len(), 1);


    let mut b = Matrix::zeros(s.rows,0);
    let mut f = Matrix::zeros(0, 0);
    let mut t = Matrix::zeros(0, 1); //TODO

    
    for (i, it) in pairs.iter().enumerate(){
        let in_node  = match unique_nodes.binary_search(&it.in_node) {
            Ok(n) => {n},
            Err(_) => { 
                println!("There was no in_node found in unique. {:?} {} {}", it, graph[it.element_index].unique_a_node, graph[it.element_index].unique_b_node);
                return (rt, vec![], Vec::new());
            },
        };
        let out_node = match unique_nodes.binary_search(&it.out_node){
            Ok(n) => {n},
            Err(_) => { 
                println!("There was no out_node found in unique. {:?} {} {} | {} {}", it, graph[it.element_index].unique_a_node, graph[it.element_index].unique_b_node, 
                                                                                           graph[it.element_index].a_node, graph[it.element_index].b_node);
                return (rt, vec![], Vec::new());
            },
        };

        *m.get_element(in_node, i) = 1.0;
        *m.get_element(out_node, i) = -1.0;
        
        *z.get_element(i, i) = -1.0* ( graph[it.element_index].resistance  + graph[it.element_index].unc_resistance );

        if graph[it.element_index].capacitance > 0.0{ //TODO should we check circuit element type instead?
            *y.get_element(i, i) = graph[it.element_index].capacitance + graph[it.element_index].unc_capacitance;
        }

        if graph[it.element_index].inductance > 0.0{ //TODO should we check circuit element type instead?
            *z.get_element(i, i) = graph[it.element_index].inductance + graph[it.element_index].unc_inductance;
            *y.get_element(i, i) = 0.0;
        }


        *s.get_element(i, 0) = if it.in_node == graph[it.element_index].a_node { graph[it.element_index].voltage + graph[it.element_index].unc_voltage } 
                               else {-1.0 * (graph[it.element_index].voltage + graph[it.element_index].unc_voltage )};

        if graph[it.element_index].circuit_element_type == SelectedCircuitElement::Capacitor{
            b.grow(0,1);
            f.grow(1,1);
            t.grow(1,0);

            let _i = t.rows-1;

            *t.get_element(_i, 0) = if it.in_node == graph[it.element_index].a_node { graph[it.element_index].charge } 
                               else { -1f32 * graph[it.element_index].charge };
            *b.get_element(i, _i) = -1.0;
            *f.get_element(_i, _i) = 1.0;
        }

        if graph[it.element_index].circuit_element_type == SelectedCircuitElement::Inductor{
            b.grow(0,1);
            f.grow(1,1);
            t.grow(1,0);

            let _i = t.rows-1;

            *t.get_element(_i, 0) = graph[it.element_index].magnetic_flux;
            *b.get_element(i, _i) = -1.0;
            *f.get_element(_i, _i)= 1.0; 
        }

        if graph[it.element_index].circuit_element_type == SelectedCircuitElement::Custom
        && graph[it.element_index].capacitance.abs() > 0.00001f32{
            b.grow(0,1);
            f.grow(1,1);
            t.grow(1,0);

            let _i = t.rows-1;

            *t.get_element(_i, 0) = if it.in_node == graph[it.element_index].a_node { graph[it.element_index].charge + graph[it.element_index].unc_charge } 
                               else { -1f32 * graph[it.element_index].charge + graph[it.element_index].unc_charge};
            *b.get_element(i, _i) = -1.0;
            *f.get_element(_i, _i) = 1.0;
        }
        if graph[it.element_index].circuit_element_type == SelectedCircuitElement::Custom
        && graph[it.element_index].inductance.abs() > 0.00001f32{
            b.grow(0,1);
            f.grow(1,1);
            t.grow(1,0);

            let _i = t.rows-1;

            *t.get_element(_i, 0) = graph[it.element_index].magnetic_flux + graph[it.element_index].unc_magnetic_flux;
            *b.get_element(i, _i) = -1.0;
            *f.get_element(_i, _i)= 1.0; 
        }

    }   

    let mut a = Matrix::zeros(m.rows-1, m.columns);
    for r in 1..m.rows{
        for c in 0..m.columns{
            *a.get_element(r-1, c) = *m.get_element(r, c);
        }
    }
    {//NOTE check matrix M for consistency
        for i in 0..m.columns{
            let mut sum = 0.0;
            for j in 0..m.rows{
                sum +=*m.get_element(j, i);
            } 
            if sum.round() != 0.0 {
                println!("Matrix M is inconsistent with definition. {:?}", m);
                return (rt, vec![], Vec::new());
            }
        } 
    }



//    println!("m: {:?}", m);
//    println!("a: {:?}", a);
//    println!("z: {:?}", z);
//    println!("s: {:?}", s);

    let mut c_matrix = Matrix::zeros( 2*a.columns + a.rows + f.rows, 2*a.columns + a.rows + b.columns + 1 ); //OLD
    {//Construct full circuit matrix
/*NOTE NEW
This is what we are trying to make:

unknowns = [  i, u,    v,  p  ]

c_matrix = [  A, 0,    0,  0, | 0
              0, I, -1*AT, 0, | 0
              Z, Y,    0,  B, | s
              0, 0,    0,  F, | t
            ]

We wish to formulate circuit equations in the matrix above and use 
Gaussian elimination to for unknowns. We begin with matrix 'A', which
contains Kerkove current loop rules, for every vertex there must be some path 
for both incoming and out going current. In the second row of c_matrix 
contains rules on the conservation of voltage. 'A' is a matrix with elements 
of 0 or 1. The next row is filled with matrices where are parameterized by 
the characteristics of circuit elements. 'Z' is related to current, and 
is where resistances might be placed. 'Y' is paired to the change in voltage 
across an element, 'B' relates to stored energy (a capacitor's charge).
It is through application of unknowns on c_matrix do we retrieve the ohm's law
and the like.  'F' is paired to store energy.

Vectors 's' and 't' can be thought of has the other half of our ohm's law like
equations. 'Z', 'Y', and 'B' deliver the right side of the equation, 's' must deliver the left.
't' sets the value of energy storage variables, as fixed in time. This must be
updated externally.

*/


        // write A
        for r in 0..a.rows{
            for c in 0..a.columns{
                let r_offset = 0;
                let c_offset = 0;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *a.get_element(r, c);
            }
        }
        // write A^T
        let mut a_t = a.transpose();
        for r in 0..a_t.rows{
            for c in 0..a_t.columns{
                let r_offset = a.rows;
                let c_offset = 2*a.columns;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *a_t.get_element(r, c) * -1.0 ;
            }
        }
        // write I
        let mut identity_m = Matrix::identity(a.columns);
        for r in 0..identity_m.rows{
            for c in 0..identity_m.columns{
                let r_offset = a.rows;
                let c_offset = a.columns;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *identity_m.get_element(r, c);
            }
        }

        //write Z
        
        for r in 0..z.rows{
            for c in 0..z.columns{
                let r_offset = a.rows + a.columns;
                let c_offset = 0;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *z.get_element(r, c);
            }
        }

        //write Y
        //NOTE for linear resistance graphs Y is I
        for r in 0..y.rows{
            for c in 0..y.columns{
                let r_offset = a.rows + a.columns;
                let c_offset = a.columns;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *y.get_element(r, c);
            }
        }

        //write B 
        for r in 0..b.rows{
            for c in 0..b.columns{
                let r_offset = a.rows + a.columns;
                let c_offset = a.columns + y.columns + a.rows;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *b.get_element(r, c);
            }
        }

        //write F
        for r in 0..f.rows{
            for c in 0..f.columns{
                let r_offset = a.rows + a.columns + z.rows;
                let c_offset = a.columns + y.columns + a.rows;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *f.get_element(r, c);
            }
        }
        
        //write s
        for r in 0..s.rows{
            for c in 0..s.columns{
                let r_offset = a.rows + a.columns;
                let c_offset = 2*a.columns + a.rows + b.columns;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *s.get_element(r, c);
            }
        }
        
        //write t 
        for r in 0..t.rows{
            for c in 0..t.columns{
                let r_offset = a.rows + a.columns + s.rows;
                let c_offset = 2*a.columns + a.rows + b.columns;
                *c_matrix.get_element( r+r_offset, c+c_offset) = *t.get_element(r, c);
            }
        }
                
    
    //println!("cmatrix\n{:?}", c_matrix);
        c_matrix.gaussian_elimination();
    }
    
    //println!("cmatrix\n{:?}", c_matrix);
    return (c_matrix, pairs, unique_nodes);
}




fn draw_graph(canvas: &mut WindowCanvas, x: &[f32], y: &[f32], rect: [i32; 4], min_x_range: f32, font_size: f32,
              mouseinfo: &MouseInfo){
    let _rect = [ rect[0] + font_size as i32 + 3, 
                  rect[1] + font_size as i32 - 5, 
                  rect[2] - font_size as i32 - 3, 
                  rect[3] - font_size as i32 + 5, 
                ];
    draw_rect(canvas, rect, C4_CREAM, true);
    draw_rect(canvas, _rect, C4_WHITE, true);

    if x.len() != y.len(){
        panic!("draw_graph x and y lens are not the same!");
    }

    let mut min_x = std::f32::MAX;
    let mut max_x = std::f32::MIN;
    for it in x.iter(){
        if *it < min_x {
            min_x = *it;
        }
        if *it > max_x {
            max_x = *it;
        }
    }
    let x_range = (max_x - min_x).abs().max(min_x_range);

    let mut min_y = std::f32::MAX;
    let mut max_y = std::f32::MIN;
    for it in y.iter(){
        if *it < min_y {
            min_y = *it;
        }
        if *it > max_y {
            max_y = *it;
        }
    }




    let _y_range = (max_y - min_y).abs();

    max_y += 1.05*_y_range.max(1.0);
    min_y -= 1.05*_y_range.max(1.0);

    let y_range = (max_y - min_y).abs().max(1.0);
    

    for i in 0..x.len(){
        let _x = (((x[i] - min_x) / x_range) * _rect[2] as f32) as i32 + _rect[0];
        let _y = (((y[i] - min_y)/ y_range) * _rect[3] as f32) as i32 + _rect[1];

        draw_rect(canvas, [_x-1, _y-1, 3, 3], C4_DGREY, true);
    }

    for i in 0..4{//Y-Axis
        let tick_label = (i as f32 / 4.0 ) * y_range + min_y;
        let _x = rect[0]-2;
        let _y = rect[1] + ((i as f32 / 4.0 ) * _rect[3] as f32) as i32;
        draw_string(canvas, &format!("{:.1}", tick_label), _x, _y, C4_BLACK, font_size);
    }

    for i in 0..4{//X-Axis
        let tick_label = (i as f32 / 4.0 ) * x_range + min_x;
        let _x = rect[0] + ((i as f32 / 4.0 ) * _rect[2] as f32) as i32 + (font_size/2f32) as i32;
        let _y = rect[1] - (font_size/2f32) as i32 + 2;
        draw_string(canvas, &format!("{:.1}", tick_label), _x, _y, C4_BLACK, font_size);
    }

    if in_rect(mouseinfo.x, mouseinfo.y, _rect){
        let _x = mouseinfo.x;
        let _y = mouseinfo.y;

        let g_x = (_x - _rect[0]) as f32 / (_rect[2] as f32) * x_range + min_x;
        let g_y = (_y - _rect[1]) as f32 / (_rect[3] as f32) * y_range + min_y;

        let _str = format!("({:.2}, {:.2})", g_x, g_y);
        let _w = get_advance_string(&_str, font_size) + 4;
        let _h = font_size as i32;

        draw_rect(canvas, [_x, _y, _w, _h], c3_to_c4(C3_CREAM, 0.5), true);
        draw_string(canvas, &_str, _x, _y, C4_BLACK, font_size);
    }
    
}


fn save_csv( filename: &str, data: &[&[f32]], data_labels: &[&str]){
    use std::io::Write;
    use std::fs::File;

    if data.len() != data_labels.len(){
        println!("data and labels have different lengths");
        return;
    }
    if data.len() == data_labels.len()
    && data.len() == 0{
        println!("There is no data");
        return;
    }
    for i in 1..data.len() {
        if data[0].len() != data[i].len(){
            println!("Data does not share the same length");
            return;
       }
    }
    //File stuff
    let mut filebuffer = match File::create(filename){
        Ok(_fb) => _fb,
        Err(_s) => {
            println!("BMP file could not be made. {}", _s);
            return;
        }
    };
    for label in data_labels.iter(){
        write!(filebuffer, "{}, ", label);
    }
    write!(filebuffer, "\n");

    for i in 0..data[0].len(){
        for j in 0..data_labels.len(){
            write!(filebuffer, "{}, ", data[j][i]);
        }
        write!(filebuffer, "\n");
    }
}


fn save_circuit_diagram(name: &str, circuit: &[CircuitElement]){unsafe{
    use std::io::prelude::*;
    use std::mem::transmute;
    use std::mem::size_of;
    //NOTE
    //save file structure
    //['c''d''# of elements in circuit array']HEADER
    //['circuit element type' 'orientation' 'x' 'y' 'length' 'unique_a_node' 'unique_b_node'
    //'resistance' 'voltage' 'current' 'capacitance' 'inductance' 'charge' 'magnetic_flux']

    let mut f = std::fs::File::create(name).expect("File could not be created.");
    f.write_all(b"CD");
    f.write_all(&transmute::<u64, [u8; size_of::<u64>()]>(size_of::<CircuitElement>() as u64));
    f.write_all(&transmute::<u64, [u8; size_of::<u64>()]>(circuit.len() as u64));

    for it in circuit.iter(){
        f.write_all( &transmute::<CircuitElement, [u8; size_of::<CircuitElement>()]>(*it) );
    }
}}

fn load_circuit_diagram(name: &str)->Vec<CircuitElement>{unsafe{
    use std::io::prelude::*;
    use std::mem::{transmute, size_of};

    let mut f = std::fs::File::open(name).expect("File could not be created.");

    let mut file_header_buffer = [0u8;2];
    f.read( &mut file_header_buffer);

    if file_header_buffer[0] as char == 'C' 
    && file_header_buffer[1] as char == 'D'{
    } else {
        panic!("File type is wrong.");
    }

//TODO
//changes need to be made so that we can change circuit struct 
//but load old versions.
//The idea is to move the read curser by sizeof_circuit_elements,
//and add new struct parameters to end of struct.
//We will need Seek or Cursor.
    let mut sizeof_circuit_elements = [0u8; size_of::<u64>()];
    f.read(&mut sizeof_circuit_elements);
    let sizeof_circuit_elements = transmute::< [u8; size_of::<u64>()], u64>(sizeof_circuit_elements);



    //NOTE check the current size of to saved size of 
    //if saved size of is larger we have a problem and 
    //things will not work
    if size_of::<CircuitElement>() < sizeof_circuit_elements as usize{
        println!("It is highly likely that the load has failed.  Current CircitElement struct is smaller than saved struct");
    }




    let mut number_circuit_elements = [0u8; size_of::<u64>()];
    f.read(&mut number_circuit_elements);

    let number_circuit_elements = transmute::< [u8; size_of::<u64>()], u64>(number_circuit_elements);
    let mut rt = Vec::with_capacity(number_circuit_elements as usize);



    let mut _it = vec![0u8; sizeof_circuit_elements as usize];
    //let mut _it = [0u8; size_of::<CircuitElement>()]; OLD Thoth Gunter 2/3/2021
    let mut max_id = 0;
    for i in 0..number_circuit_elements{
        f.read(&mut _it);

        
        let mut element  = CircuitElement::empty();
        std::ptr::copy_nonoverlapping( _it.as_mut_ptr(), &mut element as *mut CircuitElement as *mut u8, sizeof_circuit_elements as usize);

/* OLD Thoth Gunter 2/3/2021
        let it = transmute::<_, &CircuitElement>(&_it);

        let mut element  = CircuitElement::empty();
        element.circuit_element_type = it.circuit_element_type;
        element.orientation = it.orientation;
        element.x = it.x;
        element.y = it.y;
        element.length = it.length;

        element.unique_a_node = it.unique_a_node;
        element.a_node = it.a_node;
        element.unique_b_node = it.unique_b_node;
        element.b_node = it.b_node;

        element.resistance = it.resistance;
        element.voltage = it.voltage;
        element.current = it.current;

        element.unc_resistance = it.unc_resistance;
        element.unc_voltage    = it.unc_voltage;
        element.unc_current    = it.unc_current;

        element.capacitance   = it.capacitance;
        element.inductance    = it.inductance;
        element.charge        = it.charge;
        element.magnetic_flux = it.magnetic_flux;

        element.unc_capacitance   = it.unc_capacitance;
        element.unc_inductance    = it.unc_inductance;
        element.unc_charge        = it.unc_charge;
        element.unc_magnetic_flux = it.unc_magnetic_flux;

        element.max_voltage = it.max_voltage;
        element.d_voltage_over_dt = it.d_voltage_over_dt;
        element.frequency = it.frequency;
        element.ac_source_type = it.ac_source_type;

        element.label = it.label;

        element.bias = it.bias;
        element.noise = it.noise;
*/
        //TODO How are we rending things... I am confused
        if element.unique_b_node > max_id {
            max_id = element.unique_b_node;
        }
        if element.properties_selected {
            element.properties_z = get_and_update_global_properties_z();
        }
        rt.push(element);
    }

    set_unique_id(max_id+1);
    return rt;
}}




////NOTE
//The following in redundant with code from app_main, xcopy fgc ai program
#[derive(Clone)]
struct TextBox{
    text_buffer: String,
    text_cursor: usize,
    max_char: i32,
    max_render_length: i32,
    text_size: f32,
    x: i32,
    y: i32,
    text_color:[f32;4],
    bg_color:[f32;4],
    cursor_color:[f32;4],
    omega: f32,
    active: bool,

    offset_x: i32,
    offset_y: i32,
}
impl TextBox{
    pub fn new()->TextBox{
        TextBox{
            text_buffer: String::new(),
            text_cursor: 0,
            max_char: 30,
            max_render_length: 200,
            text_size: 24.0,
            x: 0,
            y: 0,
            text_color:[0.8;4],
            cursor_color:[0.8;4],
            bg_color:[1.0, 1.0, 1.0, 0.1],
            omega: 1.0f32,
            active: false,

            offset_x: 0i32,
            offset_y: 0i32,
        }
    }
    pub fn update(&mut self, keyboardinfo : &KeyboardInfo, textinfo: &TextInfo, mouseinfo: &MouseInfo){
        fn placeCursor(_self: &mut TextBox, mouseinfo: &MouseInfo){//Look for where to place cursor
            let mut position = 0;
            for (i, it) in _self.text_buffer.chars().enumerate() {
                //IF mouse is between old position and new position then we place cursor
                //behind the current character
                let adv = get_advance(it, _self.text_size);
                if i < _self.text_buffer.len() - 1 {
                    if mouseinfo.x >= position + _self.x + _self.offset_x + 4 && mouseinfo.x < position + adv + _self.x + _self.offset_x + 4 {
                        _self.text_cursor = i;
                        break;
                    }
                } else {
                    if mouseinfo.x >= position + adv + _self.x + _self.offset_x + 4 {
                        _self.text_cursor = i + 1;
                        break;
                    } else if mouseinfo.x < position + adv + _self.x + _self.offset_x + 4{
                        _self.text_cursor = i;
                        break;
                    }
                }

                position += adv;
            }
        }


        if self.active == false {
            if in_rect(mouseinfo.x, mouseinfo.y,
               [self.x+self.offset_x+4, self.y + self.offset_y + 4, self.max_render_length , self.text_size as i32]) &&
               mouseinfo.lbutton == ButtonStatus::Down{
                self.active = true;

                placeCursor(self, mouseinfo);
            }
            return;
        }


        if  self.active {
            if in_rect(mouseinfo.x, mouseinfo.y,
                [self.x+self.offset_x+4, self.y+self.offset_y + 4, self.max_render_length , self.text_size as i32]) == false &&
                mouseinfo.lbutton == ButtonStatus::Down{
                self.active = false;
                return;
            } else { //IS THIS A GOOD ELSE STATEMENT I DON'T THINK THIS MAKES SENSE
                if in_rect(mouseinfo.x, mouseinfo.y,
                   [self.x+self.offset_x+4, self.y+self.offset_y + 4, self.max_render_length , self.text_size as i32]) &&
                   mouseinfo.lbutton == ButtonStatus::Down
                {//Look for where to place cursor
                    placeCursor(self, mouseinfo);

                }

                for i in 0..keyboardinfo.key.len(){
                    if  keyboardinfo.key[i] == KeyboardEnum::Enter &&
                       keyboardinfo.status[i] == ButtonStatus::Down {
                        self.active = false;
                        return;
                    }
                }
            }
        }

        let mut delete_activated = false;
        for i in 0..keyboardinfo.key.len(){
            if keyboardinfo.status[i] == ButtonStatus::Down{
                if keyboardinfo.key[i] == KeyboardEnum::Leftarrow{
                    if self.text_cursor > 0 {
                        self.text_cursor -= 1;
                    }
                }
                if keyboardinfo.key[i] == KeyboardEnum::Rightarrow{
                    if (self.text_cursor as usize) < self.text_buffer.len() {
                        self.text_cursor += 1;
                    }
                }
                if keyboardinfo.key[i] == KeyboardEnum::Delete{
                    let _cursor = self.text_cursor;
                    delete_activated = true;
                    if self.text_buffer.len() > _cursor {
                        self.text_buffer.remove(_cursor);
                    }
                }
            }
        }

        for character in &textinfo.character{
            let _cursor = self.text_cursor as usize;

            //NOTE character with u8 of 8 is the backspace code on windows
            let u8_char = *character as u8;
            let mut is_backspace = u8_char == 8;
            if cfg!(target_os = "macos") {
                is_backspace = u8_char == 127;
            }
            
            if is_backspace && !delete_activated {
                if (self.text_buffer.len() > 0)
                && _cursor > 0 {
                    self.text_buffer.remove(_cursor-1);
                    self.text_cursor -= 1;
                } 
            } else if u8_char  >= 239 || delete_activated { //This is a delete on keyboard macos
                
            } else {
                if self.text_buffer.len() < self.max_char as usize 
                && u8_char != 8 {
                    self.text_buffer.insert(_cursor, *character);
                    self.text_cursor += 1;
                }
            }
            if self.text_cursor as usize > self.text_buffer.len() {
                self.text_cursor = self.text_buffer.len();
            }
        }

    }
    pub fn draw(&self, canvas: &mut WindowCanvas, time: f32){
        draw_rect(canvas,
             [self.x+4, self.y + 4, self.max_render_length , self.text_size as i32],
             self.bg_color, true);
        draw_string(canvas, &self.text_buffer, self.x, self.y, self.text_color, self.text_size);


        if self.active {
            let mut adv = 0;
            let mut cursor_color = self.cursor_color;
            cursor_color[3] = cursor_color[3] * ( 0.5*(self.omega*time).cos() + 0.7).min(1.0);

            for (i, it) in self.text_buffer.chars().enumerate(){

                if i == self.text_cursor as usize {
                    draw_rect(canvas, [self.x+adv+4, self.y+4, 2, self.text_size as i32],
                         cursor_color, true);
                    break;
                }
                adv += get_advance(it, self.text_size);
            }
            if self.text_buffer.len() == 0 || self.text_cursor == self.text_buffer.len(){
                draw_rect(canvas, [self.x+adv+4, self.y+4, 2, self.text_size as i32],
                     cursor_color, true);
            }
        }
    }
}

fn render_current_wire(bmp: &mut TGBitmap, count: f32){//NOTE ideas for showing current
    
    let a_width  = bmp.width as usize;
    let a_height = bmp.height as usize;

    for j in 0..a_height{
        for i in 0..a_width{
            let offset = 4*(j*a_width + i);

//TODO we are doing too much computation make cleaner
            let color_r = 255 - ((2.0 * PI * i as f32 / a_width as f32 - count).sin().powi(2)*100f32) as u8;
            let color_g = 255 - ((2.0 * PI * i as f32 / a_width as f32 - count).sin().powi(2)*255f32) as u8;;
            let color_b = 255 - ((2.0 * PI * i as f32 / a_width as f32 - count).sin().powi(2)*255f32) as u8;;

            bmp.rgba[offset + 0] = color_b;
            bmp.rgba[offset + 1] = color_g; 
            bmp.rgba[offset + 2] = color_r;

        }
    }
}

fn render_grey(bmp: &mut TGBitmap){
    
    let a_width  = bmp.width as usize;
    let a_height = bmp.height as usize;

    let color_r = 155; 
    let color_g = 155;
    let color_b = 155;

    for j in 0..a_height{
        for i in 0..a_width{
            let offset = 4*(j*a_width + i);

            bmp.rgba[offset + 0] = color_b;
            bmp.rgba[offset + 1] = color_g; 
            bmp.rgba[offset + 2] = color_r;

        }
    }
}

fn render_pink(bmp: &mut TGBitmap){
    
    let a_width  = bmp.width as usize;
    let a_height = bmp.height as usize;

    let color_r = 255; 
    let color_g = 192;
    let color_b = 203;

    for j in 0..a_height{
        for i in 0..a_width{
            let offset = 4*(j*a_width + i);

            bmp.rgba[offset + 0] = color_b;
            bmp.rgba[offset + 1] = color_g; 
            bmp.rgba[offset + 2] = color_r;

        }
    }
}

fn render_red(bmp: &mut TGBitmap){
    
    let a_width  = bmp.width as usize;
    let a_height = bmp.height as usize;

    let color_r = 155; 
    let color_g = 0;
    let color_b = 0;

    for j in 0..a_height{
        for i in 0..a_width{
            let offset = 4*(j*a_width + i);

            bmp.rgba[offset + 0] = color_b;
            bmp.rgba[offset + 1] = color_g; 
            bmp.rgba[offset + 2] = color_r;

        }
    }
}

fn render_charge_capacitor(bmp: &mut TGBitmap, charge: f32, capacitance: f32){//NOTE ideas for showing current
    //Wire
    let a_width  = bmp.width as usize;
    let a_height = bmp.height as usize;

    let x1 = [(0.375*a_width as f32) as usize, (0.45 * a_width as f32) as usize];
    let x2 = [(0.575*a_width as f32) as usize, (0.65 * a_width as f32) as usize];


    let max = 255f32;
    let max_u8 = 255;
    let half_max = 255f32 * 0.5;
    let half_max_u8 = (255f32 * 0.5) as u8;
    let x       = -4f32 + 6f32 * charge.abs() / capacitance;

    let mut r_color     = (max / (1f32+x.exp())) as u8;

    let mut color_r = 255;
    let mut color_g = r_color;
    let mut color_b = r_color;

    let mut alt_color_r = (r_color as f32 /2f32) as u8 + half_max_u8;
    let mut alt_color_g = (r_color as f32 /2f32) as u8 + half_max_u8;
    let mut alt_color_b = 255;


    if charge < 0f32{

        color_r = alt_color_r;
        color_g = alt_color_g;
        color_b = 255;

        alt_color_r = 255;
        alt_color_g = r_color;
        alt_color_b = r_color;
    }


    for j in 0..a_height{
        for i in x1[0]..x1[1]{
            let offset = 4*(j*a_width + i);


            bmp.rgba[offset + 0] = color_b;
            bmp.rgba[offset + 1] = color_g; 
            bmp.rgba[offset + 2] = color_r;

        }
        for i in x2[0]..x2[1]{
            let offset = 4*(j*a_width + i);


            bmp.rgba[offset + 0] = alt_color_b;
            bmp.rgba[offset + 1] = alt_color_g; 
            bmp.rgba[offset + 2] = alt_color_r;
        }
    }
}



fn undirected_to_directed_matrix(m : &mut Matrix){
    //NOTE rules:
    //sum of columns must be zero, sum of rows mag must be less than some value.

    //NOTE
    //1st we make a guess the direction of each pair.
    for i in 0..m.columns{
        for j in (0..m.rows).rev(){
            let mut good = false;

            if *m.get_element(j, i) == 1f32{
                /* Removed and everything seems to work...
                for k in (0..m.columns).rev(){

                    if k == i { continue; }

                    if *m.get_element(j, k) == 1f32 { 
                        good = true;
                        break; 
                    }
                }

                if good {  
                    *m.get_element( j, i ) = -1f32; 
                    break; 
                }
                */
                *m.get_element( j, i ) = -1f32; 
                break; 
            }
        }
    }
    //NOTE
    //now we try to correct that guess.
    for j in 0..m.rows{
        let mut count = 0f32;
        let mut sum = 0f32;

        let mut c_index = vec![];
        for i in 0..m.columns{
            sum   += *m.get_element(j, i);
            count += (*m.get_element(j, i)).abs();

            
            if *m.get_element(j, i) != 0f32 { c_index.push(i); }
        }

       //if j == 6 { println!("init: sum {}   count {}",sum, count); }
        if sum.abs() < count { continue; }

        //flip other element
        let mut change_made = false;
        let mut j_index = vec![];
        for _j in 0..m.rows{
            if _j == j { continue; }

            let mut row_interesting = false;
            let mut column_of_interest = 0;
            for it in c_index.iter(){
                if *m.get_element(_j, *it) != 0f32 { 
                    row_interesting = true; 
                    column_of_interest = *it; 
                }
            }

            if !row_interesting { continue; }
            j_index.push( (_j, column_of_interest) );
       //if j == 6 { println!("???"); }


            count = 0f32;
            sum = 0f32;
            for _i in 0..m.columns{
                sum   += *m.get_element(_j, _i);
                count += (*m.get_element(_j, _i)).abs();
            }
            //if j == 7 {
            //   println!("ASDF {:?}", m); 
            //   println!("ASDF {}  {:?}",sum, (j, _j, column_of_interest)); 
            //}
            //if sum.abs() < count - 2f32 {
            if sum as f32 - (*m.get_element(_j, column_of_interest)) * -1f32 != 0f32 
            && count != 2f32{
                //TODO we must now change the things
            //if j == 7 { println!("sum {} {:?}", sum, (j, _j, column_of_interest)); }
                *m.get_element(_j, column_of_interest) *=-1f32;
                *m.get_element(j, column_of_interest) *=-1f32;
                change_made = true;
            }
           //println!("ASDF {} {:?} ", j, m); 
        }
        if  !change_made {
            *m.get_element(j_index[1].0, j_index[1].1) *=-1f32;
            *m.get_element(j, j_index[1].1) *=-1f32;
        }
    }
    //panic!("{:?}", m);
}

#[test]
fn test_undir_to_directed(){
    let mut m = Matrix{
                arr: vec![
                1f32, 1f32, 0f32, 0f32,
                1f32, 0f32, 0f32, 1f32,
                0f32, 1f32, 1f32, 0f32,
                0f32, 0f32, 1f32, 1f32,
                ],
                rows: 4,
                columns: 4,};
    undirected_to_directed_matrix(&mut m);
    let mut _m = Matrix{
                arr: vec![
                1f32,-1f32, 0f32, 0f32,
               -1f32, 0f32, 0f32, 1f32,
                0f32, 1f32,-1f32, 0f32,
                0f32, 0f32, 1f32,-1f32,
                ],
                rows: 4,
                columns: 4,};
    assert_eq!(m, _m);


    let mut m = Matrix{
                arr: vec![
                1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 
                1f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 1f32, 1f32, 1f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 
                ],
                rows: 8,
                columns: 9,};
    
    undirected_to_directed_matrix(&mut m);
    let _m = Matrix{
                arr: vec![
                1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 
               -1f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 
                0f32,-1f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 0f32,-1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 0f32,-1f32,-1f32, 1f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 1f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 1f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 1f32, 
                ],
                rows: 8,
                columns: 9,};

    assert_eq!(m, _m);

    let mut m = Matrix{
                arr: vec![
                1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 0f32, 
                1f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 
                0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 1f32, 
                0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 0f32, 
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32, 1f32, 0f32, 
                ],
                rows: 8,
                columns: 9,};
    
    undirected_to_directed_matrix(&mut m);
    let _m = Matrix{
                arr: vec![
                1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 0f32,
               -1f32, 1f32, 0f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32,
                0f32,-1f32, 1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 0f32,
                0f32, 0f32,-1f32, 0f32, 0f32, 0f32, 0f32, 0f32, 1f32,
                0f32, 0f32, 0f32,-1f32, 1f32, 0f32, 0f32, 0f32,-1f32,
                0f32, 0f32, 0f32, 0f32,-1f32, 1f32, 0f32, 0f32, 0f32,
                0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 1f32, 0f32, 0f32,
                0f32, 0f32, 0f32, 0f32, 0f32, 0f32,-1f32, 1f32, 0f32,
                ],
                rows: 8,
                columns: 9,};

    assert_eq!(m, _m);
}




//TODO size needs to be a template style parameter
//TODO add to misc
const _FIXED_CHAR_BUFFER_SIZE : usize = 15;
#[derive(Clone, Copy)]
pub struct TinyString{
    //NOTE
    //currently struct vars are public for debugging purposed.  They should not be public.
    //NOTE
    //This should prob be a general tool
    pub cursor: usize,
    pub buffer: [u8; _FIXED_CHAR_BUFFER_SIZE],
}
impl TinyString{
    pub const fn new()->TinyString{
        TinyString{
            buffer: [std::u8::MAX; _FIXED_CHAR_BUFFER_SIZE],
            cursor: 0,
        }
    }
    pub fn len(&self)->usize{
        self.cursor
    }


    //TODO
    //check meaning of clone and copy in rust
    pub fn copystr(&mut self, s: &str){
        let mut bytes = s.as_bytes();

        self.cursor = 0;
        for i in 0.. bytes.len(){
            if i >= _FIXED_CHAR_BUFFER_SIZE { break; }
            self.buffer[i] = bytes[i];
            self.cursor += 1;
        }
    }

    pub fn fromstr(string: &str)->TinyString{
        let mut ts = TinyString::new();
        ts.copystr(string);
        return ts;
    }


    //TODO
    //check meaning of clone and copy in rust
    pub fn copy(&mut self, s: &TinyString){
        for i in 0..s.len(){
            //TODO
            //slow
            self.buffer[i] = s.buffer[i];
        }
        self.cursor = s.len();
    }

    pub fn is_samestr(&self, s: &str)->bool{
        let mut bytes = s.as_bytes();
        if self.len() !=  bytes.len(){ return false; }
        for i in 0..self.len(){
            if self.buffer[i] != bytes[i]{ return false; }
        }
        return true;
    }

    pub fn is_same(&self, s: &TinyString)->bool{
        if self.len() !=  s.len(){ return false; }
        for i in 0..self.len(){
            if self.buffer[i] != s.buffer[i]{ return false; }
        }
        return true;
    }
}

impl std::cmp::PartialEq for TinyString{
    fn eq(&self, other: &Self)->bool{
        self.is_same(other) 
    }
}
impl AsRef<str> for TinyString{
    fn as_ref(&self)->&str{unsafe{
        //TODO 
        //Some times this does not work wonder if it is a save fail?
        //should we breaking change things to char? I don't remeber why we didn't choose char in the first place.
        match std::str::from_utf8(std::slice::from_raw_parts(self.buffer.as_ptr() , self.cursor)){
            Ok(s)=> {s},
            Err(s)=>{ 
                println!("{:?}", s);
                "?Error?"
            }
        }
    }}
}
impl core::fmt::Display for TinyString{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->core::fmt::Result{
        core::fmt::Display::fmt(self.as_ref() ,f)
    }
}
impl core::fmt::Debug for TinyString{
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)->core::fmt::Result{
        core::fmt::Debug::fmt(self.as_ref() ,f)
    }
}






fn sample_normal(std: f32)->f32{unsafe{
    let dst = Normal::new(0.0, std*2f32.powf(-0.5)).unwrap();
    return dst.sample(&mut thread_rng());
}}









