# development
## MeiliSearch
### search
 ```shell
 curl 'http://127.0.0.1:7700/indexes/adventures_index/search' --header "X-Meili-API-Key: meili-master-key" --data '{"q":"s"}'

 curl 'http://127.0.0.1:7700/indexes/adventures_index/search' --header "X-Meili-API-Key: meili-master-key" --data '{"filter":"id=796"}'
```
### stats
```shell
 curl 'http://127.0.0.1:7700/indexes/adventures_index/stats' --header "X-Meili-API-Key: meili-master-key"

 curl 'http://127.0.0.1:7700/stats' --header "X-Meili-API-Key: meili-master-key"
```
### delete
```shell
 curl -X DELETE 'http://127.0.0.1:7700/indexes/adventures_index' --header "X-Meili-API-Key: meili-master-key"
```
### keys
```shell
 curl -X GET 'http://127.0.0.1:7700/keys' --header "X-Meili-API-Key: meili-master-key"
```
### settings
```shell
 curl -X GET 'http://127.0.0.1:7700/indexes/adventures_index/settings' --header "X-Meili-API-Key: meili-master-key"

 curl 'http://127.0.0.1:7700/indexes/adventures_index/settings/filterable-attributes' --header "X-Meili-API-Key: meili-master-key"

 curl \
  -X POST 'http://127.0.0.1:7700/indexes/adventures_index/settings' \
  --header "X-Meili-API-Key: meili-master-key" \
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
  --header "X-Meili-API-Key: meili-master-key" \
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
for i in {1..944}
do
        echo  https://adventures.jicu.vip/api/sync/$i
        curl -X GET -k --tlsv1 https://adventures.jicu.vip/api/sync/$i
        sleep 0.2
done
```