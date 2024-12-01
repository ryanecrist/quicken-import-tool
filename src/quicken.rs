use super::sqlite3;

pub struct Account {
    pub creation_timestamp: f64, // ZCREATIONTIMESTAMP
    pub id: String, // ZGUID
    pub modification_timestamp: f64, // ZMODIFICATIONTIMESTAMP
    pub name: String, // ZNAME
}

#[derive(Debug)]
pub enum Error {
    SQLite3(sqlite3::Error),
}

impl From<sqlite3::Error> for Error {
    fn from(error: sqlite3::Error) -> Self {
        return Error::SQLite3(error);
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SQLite3(error) => write!(f, "SQLite3 {error:?}")
        }
    }
}

pub struct Quicken {
    pub path: String,
    connection: Option<sqlite3::Connection>,
}

impl Quicken {

    pub fn new(path: String) -> Quicken {
        return Quicken {
            path: path,
            connection: Option::None,
        };
    }

    pub fn open(&mut self) -> Result<(), Error> {
        self.connection = Option::from(sqlite3::Connection::open(self.path.as_str())?);
        return Ok(());
    }

    pub fn close(&mut self) -> Result<(), Error> {
        if let Some(connection) = &mut self.connection {
            connection.close()?;
        }
        self.connection = None;
        return Ok(());
    }

    pub fn accounts(&self) -> Result<Vec<Account>, Error> {
        let mut accounts: Vec<Account> = Vec::new();
        if let Some(connection) = &self.connection {
            let mut statement = connection.prepare("SELECT ZCREATIONTIMESTAMP, ZGUID, ZMODIFICATIONTIMESTAMP, ZNAME FROM ZACCOUNT")?;
            while statement.step()? {
                let creation_timestamp = statement.column_double(0)?;
                let id = statement.column_text(1)?;
                let modification_timestamp = statement.column_double(2)?;
                let name = statement.column_text(3)?;
                accounts.push(Account { 
                    creation_timestamp: creation_timestamp,
                    id: id,
                    modification_timestamp: modification_timestamp,
                    name: name,
                });
            }
            statement.finalize()?;
        }
        return Ok(accounts);
    }
}