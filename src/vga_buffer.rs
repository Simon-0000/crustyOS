use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color{
    Black=0,
    Blue=1,
    Green=2,
    Cyan=3,
    Red=4,
    Magenta=5,
    Brown=6,
    LightGray=7,
    DarkGray=8,
    LightBlue=9,
    LightGreen=10,
    LightCyan=11,
    LightRed=12,
    Pink=13,
    Yellow=14,
    White=15,
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode{
    fn new(forground:Color, backGround:Color) ->ColorCode {
        ColorCode((backGround as u8) <<4 | (forground as u8))
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(C)]//to make sure that it follows c conventions for ordering
struct ScreenChar{
    ascii : u8,
    color_code: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
#[repr(transparent)]//to make sure that it respects the same ScreenChar size behaviour 
struct Buffer{
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer{
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}


impl Writer{
    pub fn write_byte(&mut self, byte:u8){
        match byte{
            b'\n'=>self.new_line(),
            _=> {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii: byte,
                    color_code: self.color_code
                });
                self.column_position +=  1;
            }
        }
    }
    pub fn write_string(&mut self, message:&str){
        message.as_bytes().iter().for_each(|&c|
            match c {
                0x20..=0x7e | b'\n' => self.write_byte(c),
                _=>self.write_byte(0xfe)
            }
        )   
    }


    fn new_line(&mut self) {
        for i in 1..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                self.buffer.chars[i-1][j].write(self.buffer.chars[i][j].read());
            }
        }
        self.clear_line(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    fn clear_line(&mut self, line : usize) {
        self.buffer.chars[line].iter_mut().for_each(|v| 
            v.write(ScreenChar{ascii:b' ',color_code: self.color_code}));
    }
}

impl fmt::Write for Writer
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}