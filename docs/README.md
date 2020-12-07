# Documentation


## Datatypes

### Guild
| Name     | Type    | Description |
|----------|---------|-------------|
| name     | string  |             |
| id       | int     |             |
| icon_url | string  |             |
| profiles | Profile |             |

### Profile
| Name          | Type   | Description |
|---------------|--------|-------------|
| tag           | string |             |
| avatar_url    | string |             |
| github        | string |             |
| contributions | int    |             |

## Endpoints
| Endpoint        | Return Type | Requires Login | Description |
|-----------------|-------------|----------------|-------------|
| /api/user       | Profile     | *              |             |
| /api/guilds     | Guild       | *              |             |
| /oauth          | null        |                |             |
| /oauth/callback | null        |                |             |



