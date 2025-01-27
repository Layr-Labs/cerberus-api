# cerberus-api
This is the API spec of remote signer. 
The spec currently only support BLS on bn254 signing. 

## Disclaimer
🚧 Cerberus-api is under active development and has not been audited.
Cerberus-api is rapidly being upgraded, features may be added, removed or otherwise improved or modified and interfaces will have breaking changes.
Cerberus-api should be used only for testing purposes and not in production. Cerberus-api is provided "as is" and Eigen Labs, Inc. does not guarantee its functionality or provide support for its use in production. 🚧

## Supported Bindings
### Go
The go bindings resides in [pkg/api/vi](pkg/api/v1) directory.

### Rust
The rust bindings resides in [src](src) directory.

## Signing Quirks
If you are implementing a version of this, please make sure to check [this code](https://github.com/Layr-Labs/cerberus/blob/6ce641c6323c412b2b9383169ee70fef22c13c60/internal/crypto/utils.go#L30-L36) 
for implementation of sign and verify. If you use any other implementation, the signatures will not be compatible with EigenLayer contracts.
Eventually we will support more `HashToCurve` algorithms.

## Implementation
* Go - https://github.com/Layr-Labs/cerberus
  
## Usage
### Signing Client
```go
package main

import (
    "context"
    "fmt"
    "log"
    "time"

    "github.com/Layr-Labs/cerberus-api/pkg/api/v1"
	
    "google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
)

const SIGNER_API_KEY = "<API-KEY>"

func main() {
	conn, err := grpc.NewClient(
		"localhost:50051", 
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
    if err != nil {
        log.Fatalf("did not connect: %v", err)
    }
    defer conn.Close()
	
    c := v1.NewSignerClient(conn)

    ctx, cancel := context.WithTimeout(context.Background(), time.Second)
    defer cancel()

    req := &v1.SignGenericRequest{
        PublicKeyG1: "0xabcd",
        Password:  "p@$$w0rd",
        Data:      []byte{0x01, 0x02, 0x03},
    }

    // Pass the API key to the signer client
    ctx = metadata.AppendToOutgoingContext(ctx, "authorization", SIGNER_API_KEY)
    resp, err := c.SignGeneric(ctx, req)
    if err != nil {
        log.Fatalf("could not sign: %v", err)
    }
    fmt.Printf("Signature: %v\n", resp.Signature)
}
```

## Security Bugs
Please report security vulnerabilities to security@eigenlabs.org. Do NOT report security bugs via Github Issues.