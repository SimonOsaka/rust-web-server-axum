## authorization

use casbin-rs

### ROLE

```mermaid
graph BT;
  DATA1 --> ADMIN
  DATA2 --> ADMIN
  VIEW1 --> DATA1
  VIEW2 --> DATA2
```