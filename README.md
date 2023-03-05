# rust_rest_api

Rust with Actix web framework


First api would be manager for TDS log
https://dpcalfola.tistory.com/entry/Task-driven-Study-Log

<hr>

### Developed Feature List

* Build
  * Success with 1 warning

* Request:
  * get("/")
    * Response
      * Status 200 OK
      * "This is the message to check if the server is running."


* Internal DB
  *  TicketListEntry {
     id: u32,
     ticket_code_main: String,
     ticket_code_sub: String,
     ticket_subject: String,
     publishing_date: i64,
     publishing_text: String,
     publishing_link: String,
     publishing_subtext: String,
     on_progress: bool,
     completed: bool,
     completed_date: i64,
     completed_text: String,
     completed_link: String,
     completed_subtext: String,
     }


  
