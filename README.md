# autotinder

Tinder is 99% a waste of time. A person can't spend his leisure time swiping right to get a chance to sleep with someone. Moreover, Tinder does not provide proper API documentation, and thus we need to understand how the frontend communicates with the backend to automatize the "swipe right" process.

I found two useful endpoints. In both cases you need to provide a `x-auth-token` that you can easily find by opening the developer tools (on the Network tab) of your favorite browser and interacting with the web application of Tinder. 

- https://api.gotinder.com/v2/recs/core
which accepts GET requests and provides a list of recommended users in your area;

- https://api.gotinder.com/like/{id}
which accepts POST requests and "swipes right" on the provided user id. The payload is something like this
```
{
"s_number": 00034345340,
"liked_content_id": "aaaa-74747474-ryryr",
"liked_content_type", "photo",
}
```
While `liked_content_id` is the id of a photo of the user, I don't know what `s_number` means. Anyway, both information can be found in the response to the above mentioned GET request.

## Rust 
The code is *pretty crappy*, and I'm using this Tinder shit to teach myself Rust the hard way, i.e. actually doing things and face real world problems beyond the classic textbook examples. 

###### Build
```
cargo build
```

###### Run
```
./target/debug/autotinder [x-auth-token]
```