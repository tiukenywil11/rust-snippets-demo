// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum TicketType {
    Backstage(String),
    Vip(String),
    Standard
} 

#[derive(Debug)]
struct Ticket {
    ticket_type: TicketType,
    price: i32
}

pub fn advanced_match() {

    let backstage_ticket = Ticket {
        ticket_type: TicketType::Backstage("Kenny".to_owned()),
        price: 1000
    };

    let vip_ticket = Ticket {
        ticket_type: TicketType::Vip("Kenny".to_owned()),
        price: 500
    };

    let standard_ticket = Ticket {
        ticket_type: TicketType::Standard,
        price: 100
    };

    let tickets = vec![
        backstage_ticket,
        vip_ticket,
        standard_ticket
    ];

    for ticket in tickets {
        match ticket {
            _ => println!("{:?}", ticket)
        }
    }
}