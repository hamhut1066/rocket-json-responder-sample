# Example Application showing Json header clobbering

The application exposes two endpoints:

_/test_ which works fine

_/test2_ which loses header information.

The working version has the following type:

MyResponder<Json<Emptyresponse>>

The Broken version

Json<MyResponder<EmptyResponse>>


EmptyResponse is an empty struct, and MyResponder simply attaches a header
