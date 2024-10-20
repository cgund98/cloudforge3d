# AWS CDK Resources

We use the AWS CDK to deploy our cloud resources.

## Deploying

### Set up Virtual Env

To manually create a virtualenv on MacOS and Linux:

```bash
python3 -m venv .venv
```

After the init process completes and the virtualenv is created, you can use the following
step to activate your virtualenv.

```bash
source .venv/bin/activate
```

If you are a Windows platform, you would activate the virtualenv like this:

```powershell
.venv\Scripts\activate.bat
```

Once the virtualenv is activated, you can install the required dependencies.

```bash
$ pip install -r requirements.txt
```

### Applying commands

You will have to bootstrap the project when you run the CDK for the first time.

```bash
cdk bootstrap
```

At this point you can now synthesize the CloudFormation template for this code.

```bash
cdk synth
```

Finally, you can deploy the resources.

```bash
cdk deploy
```
