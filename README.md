This file provides the code for an aws lambda that
performs a single rust postgresql query and returns the
results in JSON

It can be used for reference for anyone who needs
the same thing

in the future I hope to make it generic
in that users can supply the query to execute
and credentials and get JSON results without
much effort

to use yourself you can set up your env like .env.example with
credentials and modify the query and return type

Uses cargo-lambda so first run aws configure.

test locally with
cargo lambda watch 
deploy with 
cargo lambda build
cargo lambda deploy --s3-bucket bucket-name
