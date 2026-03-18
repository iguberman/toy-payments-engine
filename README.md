# toy-payments-engine
Simple Transaction Processor

> cargo build

> cargo run -- csv_file_path


**Solution is supposed to be complete for all 5 types:**
* `deposit`  -> always work unless account is locked
* `withdrawal` -> only works if account exists, has enough funds, and is not locked
* `dispute` -> only works if the `deposit` transaction exists, account exists and is not locked
* `resolve` -> only works if the `dispute` transaction exists, account exists and is not locked
* `chargeback` -> only works if the `dispute` transaction exists, account exists and is not locked

Upon `chargeback` account is frozen for good and can't do anything at all with subsequent entries referring to it.
ALl errors go to stderr.  

NOTE: that only transaction in its current state is stored in memory, no historical transition records. 

### Testing
Manually tested with test csv input and output files is in `test_csv` folder.   It was the easiest and fastest way.  
Definitely not the best.  If this wasn't a toy project, I would have written extensive tests to cover at least all logical cases.

### AI Use

In a separate crate, I had Claude write transaction test data generator with a lot of parameters, like percentage of errors, number of rows, number of clients, holding off on chargebacks, etc. etc.
That was mostly done to test large quantities with properly generated client_id `u16` and tx_id `u32`, not so much for correctness.  It generated 100_000-row input CSV and saw that my tx_processor went though it very fast with no fatal errors.  
This generator is a very crude development and I am not including it here, but it helped me verify I can run a CSV with 100,000 rows and 20,000 clients of prod-like CSV data to very roughly assess how my single-threaded program handles performance. 
It was the fastest way to test bulk data processing. 

Also I used Claude for small routine subtasks at the beginning of this effort, like CSV parsing, but then simplified and changed it anyway.  There might be some AI-looking comments still around.  For program logic I didn't use AI at all because I wanted to use **Rust type state** pattern and it was just easier and more fun to do it myself.  I did a decent job keeping it well organized, small, and readable, but suspect that my approach to transaction state transitions could be further simplified.

NOTE: This processor is single-threaded for simplicity.  In production I would:

* at the very least shard the clients to be able to paralellize this entire effort to multiple tx_processors.  
* use a persistent store and a cache for transactions and accounts  
* obviously use log rather than `println!` or `eprintln!`


