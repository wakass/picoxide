use arrayvec::ArrayVec;

pub const CMD_BUF_SIZE: usize = 64;

pub struct Controller {
    pub cmd_buf: ArrayVec<u8, CMD_BUF_SIZE>,
}

pub struct ControllerBuilder {
    cmd_buf: ArrayVec<u8, CMD_BUF_SIZE>,
}

impl ControllerBuilder {
    pub fn new() -> ControllerBuilder {
        ControllerBuilder {
            cmd_buf: ArrayVec::<_, CMD_BUF_SIZE>::new(),
        }
    }

    pub fn cmd_buf(mut self, vec: ArrayVec<u8, CMD_BUF_SIZE>) -> ControllerBuilder {
        self.cmd_buf = vec;
        self
    }
    pub fn build(self) -> Controller {
        Controller {
            cmd_buf: self.cmd_buf,
        }
    }
}
