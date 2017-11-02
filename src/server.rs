use mysql::Conn;
use mysql::Opts;
use mysql::Error;

pub struct Server {
    conn: Conn
}

#[derive(Debug)]
pub struct TransactionResult {
    pub waiting_trx_id : Option<String>,
    pub waiting_thread : Option<u32>,
    pub waiting_query : Option<String>,
    pub waiting_for_lock : Option<String>,
    pub blocking_trx_id : Option<String>,
    pub blocking_thread : Option<u32>,
    pub blocking_query : Option<String>,
    pub blocking_lock : Option<String>,
}

impl Server {
    pub fn new<O>(opts : O) -> Result<Server, Error> 
    where O : Into<Opts> {
        Ok(Server {
            conn: Conn::new(opts.into())?
        })
    }

    pub fn conn(&mut self) -> &mut Conn {
        &mut self.conn
    }

    pub fn get_transactions(&mut self) -> Vec<TransactionResult>
    {
        self.conn().prep_exec(r#"
SELECT 
r.trx_id waiting_trx_id,
r.trx_mysql_thread_id waiting_thread,
r.trx_query as waiting_query, -- this is real 
r.trx_started as read_trx_started,
concat(concat(lw.lock_type, ' '), lw.lock_mode) waiting_for_lock,
b.trx_id blocking_trx_id,
b.trx_mysql_thread_id blocking_thread,
b.trx_query as blocking_query, -- this is just current,
b.trx_started as blocking_trx_started,
concat(concat(lb.lock_type, ' '), lb.lock_mode) blocking_lock 
FROM information_schema.innodb_lock_waits w 
INNER JOIN information_schema.innodb_trx b ON b.trx_id = w. blocking_trx_id 
INNER JOIN information_schema.innodb_trx r ON r.trx_id = w. requesting_trx_id 
INNER JOIN information_schema.innodb_locks lw ON lw.lock_trx_id = r. trx_id 
INNER JOIN information_schema.innodb_locks lb ON lb.lock_trx_id = b. trx_id
WHERE b.trx_started <= (NOW() - INTERVAL 3 SECOND)
        "#, ()).map(|result| {

            result.map(|x| x.unwrap()).map(|mut row| {
                TransactionResult {
                    waiting_trx_id : row.take("waiting_trx_id").expect("waiting_trx_id"),
                    waiting_thread : row.take("waiting_thread").expect("waiting_thread"),
                    waiting_query : row.take("waiting_query").expect("waiting_query"),
                    waiting_for_lock : row.take("waiting_for_lock").expect("waiting_for_lock"),
                    blocking_trx_id : row.take("blocking_trx_id").expect("blocking_trx_id"),
                    blocking_thread : row.take("blocking_thread").expect("blocking_thread"),
                    blocking_query : row.take("blocking_query").expect("blocking_query"),
                    blocking_lock : row.take("blocking_lock").expect("blocking_lock"),
                }

            }).collect()
        }).unwrap()
    }
}