This is how I deploy the Conformulator:
```
$ $ aws ec2 run-instances --region us-east-1 --image-id ami-5cc39523 --count 1 --instance-type t2.micro --iam-instance-profile Arn=arn:aws:iam::462876742192:instance-profile/rust-conformulator --subnet-id subnet-c654bc9d --security-group-ids sg-0206234a --tag-specifications 'ResourceType=instance,Tags=[{Key=Name,Value=rust-conformulator}]' --associate-public-ip-address --user-data file://instance-init
```

There's clearly some AWS magic in there that is specific to my account. If the purpose of some of that command is unclear, open a ticket and ask.

Most of the real magic is in the [instance-init file](instance-init), which currently bootstraps the machine to the point that the service is built and ready, but not running.

# Creating the config.toml.asc File
```
$ aws kms encrypt --key-id f3f65e1b-39c4-4e65-a2b0-4b28adc26b59 --plaintext fileb://config.toml --output text --query CiphertextBlob | base64 --decode > config.toml.secure
```
