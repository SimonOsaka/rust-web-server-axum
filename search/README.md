# development
## MeiliSearch v0.25.2
### search
 ```shell
 curl 'http://127.0.0.1:7700/indexes/adventures_index/search' --header "Authorization: Bearer meili-master-key" -H 'Content-Type: application/json' --data-binary '{"q":"s"}'

 curl 'http://127.0.0.1:7700/indexes/adventures_index/search' --header "Authorization: Bearer meili-master-key" -H 'Content-Type: application/json' --data-binary '{"filter":"id=796"}'
```
### stats
```shell
 curl 'http://127.0.0.1:7700/indexes/adventures_index/stats' --header "Authorization: Bearer meili-master-key"

 curl 'http://127.0.0.1:7700/stats' --header "Authorization: Bearer meili-master-key"
```
### delete
```shell
 curl -X DELETE 'http://127.0.0.1:7700/indexes/adventures_index' --header "Authorization: Bearer meili-master-key"
```
### keys
```shell
 curl -X GET 'http://127.0.0.1:7700/keys' --header "Authorization: Bearer meili-master-key"
```
### settings
```shell
 curl -X GET 'http://127.0.0.1:7700/indexes/adventures_index/settings' --header "Authorization: Bearer meili-master-key"

 curl 'http://127.0.0.1:7700/indexes/adventures_index/settings/filterable-attributes' --header "Authorization: Bearer meili-master-key"

 curl \
  -X POST 'http://127.0.0.1:7700/indexes/adventures_index/settings' \
  --header "Authorization: Bearer meili-master-key" \
  -H 'Content-Type: application/json' \
  --data '{
      "filterableAttributes": [
          "id",
          "play_list",
          "is_deleted",
          "item_type",
          "journey_destiny"
      ]
  }'

 curl \
  -X POST 'http://127.0.0.1:7700/indexes/adventures_index/settings' \
  --header "Authorization: Bearer meili-master-key" \
  -H 'Content-Type: application/json' \
  --data '{
      "sortableAttributes": [
          "id"
      ]
  }' 
```

## Sync from db to meilisearch
```shell
#!/bin/bash
for i in {1..1000}
do
        echo  https://adventures.jicu.vip/api/sync/$i
        curl -X GET -H 'Authorization: Token <replace your login token>' -k --tlsv1 https://adventures.jicu.vip/api/sync/$i
        sleep 0.2
done
```