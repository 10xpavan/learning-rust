//lets define enum, which lists the prossible variation of something.
fn main(){
enum ipkinder {
    V4,
    V6,                    //so as we know ip address can either be version 4 or 6, so now since we dont know how to store ip data, we only know waht kid it is, we will use structs to, make a structure to store ip data.
}

struct ipstore {
    kind: ipkinder,
    address: String,              //we have the structre of how to store the ip.
}

let home = ipstore {
    kind: ipkinder::V4,
    address: String::from("127.0.0.1"),           //the reason we are using string here isbeucase to store the ip with all the dots between and as it is in the heap memeory, we dont want it to be converted into binary, idk why.
};

let loopback = ipstore {
    kind: ipkinder::V6,
    address: String::from("::1"),
}

}