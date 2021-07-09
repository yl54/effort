// This file exists to test out the securecookie library.
package main

// imports 
import (
    "fmt"

    "github.com/gin-contrib/sessions"
    "github.com/gin-contrib/sessions/cookie"
)

// main
func main() {
    // call the test function
    hashKeyTests()
}

// function to make a hash key
func hashKeyTests() {
    // make a set of inputs
    input := []string{"17d4e712c41d95d3ad6972b00c7b87bc", "Hph7bg5SiKgVIXyAlvLIpAOs_RV42314"}

    // do a for loop over the input values
    for _, v := range input {
        fmt.Printf("v: %s\n", v)

        // try to invoke the cookie method
        store := cookie.NewStore([]byte(v))
        sessions.Sessions("pm-sesson", store)
        
    }
}