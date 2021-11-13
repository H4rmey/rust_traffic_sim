pub mod inputBuffer;
pub mod road;

use inputBuffer::InputBuffer;
use road::Road;

pub struct Crossing
{
    inputs      : Vec<InputBuffer>,
    roads       : Vec<Road>,
}

impl Crossing
{
    pub fn new()
    {
        println!("CasIsGay!");
    }
}