
use crate::serial::{Serial, TestSerial};


const START: u8 = b'#';
const END: u8 = b'e';
/**
 * start character
 * type
 * custom data
 * checksum
 * end character
 */
pub struct Protocol1<S: Serial, const SIZE: usize> {
    serial: S,
    buffer: [u8; SIZE],
    pos: u8,
}
#[derive(Debug, PartialEq, Eq)]
enum Data1 {
    ResponseOk(u8),
    JoistickData(u8, u8, u8),
}


impl From<&Data1> for Vec<u8> {
    fn from(value: &Data1) -> Self {
        let mut ret = Vec::new();
        match value {
            Data1::JoistickData(code, x, y) => {
                ret.push(0);
                ret.push(*code);
                ret.extend(x.to_le_bytes());
                ret.extend(y.to_le_bytes());
            }
            Data1::ResponseOk(code) => {
                ret.push(1);
                ret.push(*code);
            }
        }
        ret
    }
}
impl TryFrom<Vec<u8>> for Data1 {
    type Error=();

    fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
        //println!("{v:?}");
        match (v[0], v.len()){
            //(0, _)=>todo!(),
            (1, 2)=>{
                Ok(Data1::ResponseOk(v[1]))
            },
            _=>{Err(())}
        }
        
        //Err(())
    }
}

impl<S: Serial, const SIZE: usize> Protocol1<S, SIZE> {
    pub fn new(serial: S) -> Self {
        Self {
            serial,
            buffer: [0; SIZE],
            pos: 0,
        }
    }
    pub fn send<T: Into<Vec<u8>>>(&mut self, to_send: T) {
        let t: Vec<u8> = to_send.into();
        let len = t.len();
        self.serial.send(START);
        let mut checksum: u8 = 0;
        for i in t {
            checksum = checksum.wrapping_add(i);
            self.serial.send(i);
        }
        self.serial.send(checksum);
        self.serial.send(len as u8);
        self.serial.send(END);
    }

    fn read_next_char(&mut self)->Option<u8>{
        let readen = self.serial.receive()?;
        let index = self.pos as usize;
        self.buffer[index]=readen;
        self.pos=((index+1)%SIZE) as u8;
        //println!("{}", index+1);
        Some(readen)
    }

    pub fn read<T: TryFrom<Vec<u8>>>(&mut self) -> (Option<T>, bool) {
        let mut next=None;
        while let Some(x) = self.read_next_char(){
            //println!("[{x}]");
            next= Some(x);
        }
        //println!("last: {:?}", next);
        if next.is_none(){
            return (None, false);
        }
        let next = next.unwrap();
        let length = (self.pos as usize +SIZE-1)%SIZE; 
        let length = self.buffer[length] as usize;
        let end = (self.pos as usize +SIZE-3)%SIZE;
        let start = (self.pos as usize +256*SIZE-length-4)%SIZE;
        if next!=END||self.buffer[start]==START{
            return (None, true);
        }
        let mut v= Vec::new();
        let mut cur: usize = start;
        let mut chksm: u8=0;
        //println!("{start}, {end}");
        while cur!=end{
            v.push(self.buffer[cur]);
            chksm=chksm.wrapping_add(self.buffer[cur]);
            cur=(cur+1)%SIZE;
        }
        if chksm!=self.buffer[end]{
            //println!("buffer mismatch {} {}", chksm, self.buffer[end]);
            (None, true)
        }else{
            (T::try_from(v).ok(), true)
        }
        
    }
}
pub fn test(){
    
    let (serial1, serial2) = TestSerial::new(0.1);
    let mut send: Protocol1<TestSerial, 100> = Protocol1::new(serial1);
    let mut read: Protocol1<TestSerial, 100> = Protocol1::new(serial2);
    let mut correct=0;
    let mut false_positive=0;
    let mut tot=0;
    for i in 0..1000000{
        tot+=1;
        let z = Data1::ResponseOk((i%256) as u8);
        send.send(&z);
        let (content, have_read)=read.read::<Data1>();
        if content.is_some(){
            if content==Some(z){
                correct+=1;
            }else{
                false_positive+=1;
            }
        }
        /*if readen==(Some(z), true){
            correct+=1;
        }else{
           //panic!("wtf")
        }*/
    }
    
    /*for i in 0..6 {
        let x = read.serial.receive();
        println!("{:?}", x);
    }*/
    
    println!("correct = {correct:?}/{tot}");
    println!("false positive = {false_positive:?}/{tot}");
}