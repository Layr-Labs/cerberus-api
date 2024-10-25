## cerberus-api
This is the API spec of remote signer. 
The spec currently only support BLS on bn254 signing. 

## Supported Bindings
### Go
The go bindings resides in [pkg/api/vi](pkg/api/v1) directory.

## Signing Quirks
If you are implementing a version of this, please make sure to check [this code](https://github.com/Layr-Labs/remote-bls/blob/55a19a0386edcee1b5c2ecae116ae468a6b2b47b/internal/crypto/utils.go#L30-L36) 
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
		PublicKey: "0xabcd",
		Password:  "p@$$w0rd",
		Data:      []byte{0x01, 0x02, 0x03},
    }
    resp, err := c.SignGeneric(ctx, req)
    if err != nil {
        log.Fatalf("could not sign: %v", err)
    }
    fmt.Printf("Signature: %v\n", resp.Signature)
}
```

## Security Bugs
Please report security vulnerabilities to security@eigenlabs.org. Do NOT report security bugs via Github Issues.