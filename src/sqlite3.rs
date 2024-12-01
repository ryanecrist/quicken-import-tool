include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(Debug)]
pub enum Error {
    FFI(std::os::raw::c_int),
}

pub struct Statement {
    stmt: *mut sqlite3_stmt,
}

impl Statement {

    pub fn finalize(&mut self) -> Result<(), Error> {
        let code: std::os::raw::c_int;
        unsafe {
            code = sqlite3_finalize(self.stmt);
        }
        if code == SQLITE_OK as std::os::raw::c_int {
            self.stmt = std::ptr::null_mut();
            return Ok(());
        }
        return Err(Error::FFI(code));
    }

    pub fn reset(&self) -> Result<(), Error> {
        let code: std::os::raw::c_int;
        unsafe {
            code = sqlite3_reset(self.stmt);
        }
        if code == SQLITE_OK as std::os::raw::c_int {
            return Ok(());
        }
        return Err(Error::FFI(code));
    }

    pub fn step(&self) -> Result<bool, Error> {
        let code: std::os::raw::c_int;
        unsafe {
            code = sqlite3_step(self.stmt);
        }
        if code == SQLITE_DONE as std::os::raw::c_int {
            return Ok(false);
        } else if code == SQLITE_ROW as std::os::raw::c_int {
            return Ok(true);
        }
        return Err(Error::FFI(code))
    }

    pub fn column_double(&self, col: u32) -> Result<f64, Error> {
        unsafe {
            let double = sqlite3_column_double(self.stmt, col as i32);
            return Ok(double);
        }
    }
    
    pub fn column_text(&self, col: u32) -> Result<String, Error> {
        unsafe {
            let text = sqlite3_column_text(self.stmt, col as i32);
            if text != std::ptr::null() {
                let c_text = std::ffi::CStr::from_ptr(text as *const i8);
                return Ok(String::from(c_text.to_str().unwrap())); // TODO: don't force unwrap
            }
        }
        return Ok(String::from(""));
    }
}

impl Drop for Statement {
    fn drop(&mut self) {
        self.finalize().expect("failed to finalize statement");
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::FFI(code) => write!(f, "FFI {}", code)
        }
    }
}

pub struct Connection {
    db: *mut sqlite3,
}

impl Connection {

    pub fn open(filename: &str) -> Result<Self, Error> {
        let c_filename = std::ffi::CString::new(filename).expect("could not create c-string file name");
        let mut db: *mut sqlite3 = std::ptr::null_mut();
        let code: std::os::raw::c_int;
        unsafe {
            code = sqlite3_open(c_filename.as_ptr(), &mut db);
        }
        if code == SQLITE_OK as std::os::raw::c_int {
            return Ok(Connection { db: db });
        }
        return Err(Error::FFI(code));
    }

    pub fn close(&mut self) -> Result<(), Error> {
        let code: std::os::raw::c_int;
        unsafe {
            code = sqlite3_close(self.db);
        }
        if code == SQLITE_OK as std::os::raw::c_int {
            self.db = std::ptr::null_mut();
            return Ok(());
        }
        return Err(Error::FFI(code));
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement, Error> {
        let mut stmt: *mut sqlite3_stmt = std::ptr::null_mut();
        let code: std::os::raw::c_int;
        unsafe {
            let c_sql = std::ffi::CString::new(sql).expect("could not create c-string sql");
            code = sqlite3_prepare_v2(self.db, c_sql.as_ptr(), sql.len() as std::os::raw::c_int, &mut stmt, std::ptr::null_mut());
        }
        if code == 0 {
            return Ok(Statement { stmt: stmt });
        }
        return Err(Error::FFI(code));
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // TODO: finalize all statements
        self.close().expect("failed to close connection");
    }
}