# development
## MeiliSearch
### search
 ```shell
 curl 'http://192.168.56.102:7700/indexes/adventures_index/search' --header "X-Meili-API-Key: meili-master-key" --data '{"q":"s"}'

 curl 'http://192.168.56.102:7700/indexes/adventures_index/search' --header "X-Meili-API-Key: meili-master-key" --data '{"filter":"id=796"}'
```
### stats
```shell
 curl 'http://192.168.56.102:7700/indexes/adventures_index/stats' --header "X-Meili-API-Key: meili-master-key"

 curl 'http://192.168.56.102:7700/stats' --header "X-Meili-API-Key: meili-master-key"
```
### delete
```shell
 curl -X DELETE 'http://192.168.56.102:7700/indexes/adventures_index' --header "X-Meili-API-Key: meili-master-key"
```
### keys
```shell
 curl -X GET 'http://192.168.56.102:7700/keys' --header "X-Meili-API-Key: meili-master-key"
```
### settings
```shell
 curl -X GET 'http://192.168.56.102:7700/indexes/adventures_index/settings' --header "X-Meili-API-Key: meili-master-key"

 curl 'http://192.168.56.102:7700/indexes/adventures_index/settings/filterable-attributes' --header "X-Meili-API-Key: meili-master-key"

 curl \
  -X POST 'http://192.168.56.102:7700/indexes/adventures_index/settings' \
  --header "X-Meili-API-Key: meili-master-key" \
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
  -X POST 'http://192.168.56.102:7700/indexes/adventures_index/settings' \
  --header "X-Meili-API-Key: meili-master-key" \
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
        echo  https://adventures.jicu.vip/api/adventures/$i
        curl -X GET -k --tlsv1 https://adventures.jicu.vip/api/adventures/$i
        sleep 0.2
done
```