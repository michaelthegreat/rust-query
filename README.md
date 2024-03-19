This file provides the code for an aws lambda that
performs a single rust postgresql query and returns the
results in JSON

It can be used for reference for anyone who needs
the same thing

to use yourself you can set up your env like .env.example with
credentials and modify the query and return type

Uses cargo-lambda so first run aws configure.

test locally with

cargo lambda watch 

deploy with 

cargo lambda build

cargo lambda deploy --s3-bucket bucket-name

If it can be made more generic and serialize the json at the end in an automated way I think it would be more useful . Like, if it took any sql query as input and just figured out the types on its own I think it would have great power to help developers get a sql query out of a lambda quickly. Let me know if you know tips how to do this :)




get around wsl issue with cargo lambda
(for some reason it cant find ziglang unless you run this.
It doesnt even reinstall but it fixes the path somehow)
eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"
brew install cargo-lambda
