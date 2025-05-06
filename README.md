# AllowBlockList Token

An example of a allow / block list token using token extensions.

## Features

Initializes new tokens with several configuration options:
- Permanent delegate
- Allow list
- Block list
- Metadata
- Authorities

The issuer can configure the allow and block list with 3 distinct configurations:
- Force Allow: requires everyone receiving tokens to be explicitly allowed in
- Block: allows everyone to receive tokens unless explicitly blocked
- Threshold Allow: allows everyone to receive tokens unless explicitly blocked up until a given transfer amount threshold. Transfers larger than the threshold require explicitly allow

These configurations are saved in the token mint metadata.