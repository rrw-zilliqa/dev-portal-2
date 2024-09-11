---
id: APIs/1123f1a7/api/zilliqa/GetCurrentDSEpoch
title: GetCurrentDSEpoch
keywords: DS,epoch,get,current
---

---

!!! warning

    This API is not yet implemented in this version of Zilliqa 2.0

Returns the current number of DS blocks in the network. This is represented as a
`String`.

### Example Request

=== "cURL"

    ```shell
    curl -d '{
        "id": "1",
        "jsonrpc": "2.0",
        "method": "GetCurrentDSEpoch",
        "params": [""]
    }' -H "Content-Type: application/json" -X POST "https://api.zq2-devnet.zilliqa.com"
    ```

=== "node.js"

    ```js
    const currentDSEpoch = await zilliqa.blockchain.getCurrentDSEpoch();
    console.log(currentDSEpoch.result);
    ```

=== "java"

    ```java
    public class App {
        public static void main(String[] args) throws IOException {
            HttpProvider client = new HttpProvider("https://api.zq2-devnet.zilliqa.com");
            Rep<String> currentDSEpoch = client.getCurrentDSEpoch();
            System.out.println(new Gson().toJson(currentDSEpoch));
        }
    }
    ```

=== "python"

    ```python
    from pyzil.zilliqa import chain
    chain.set_active_chain(chain.MainNet)
    print(chain.active_chain.api.GetCurrentDSEpoch())
    ```

=== "go"

    ```go
    func GetCurrentDSEpoch() {
      provider := NewProvider("https://api.zq2-devnet.zilliqa.com")
      response := provider.GetCurrentDSEpoch()
      result, _ := json.Marshal(response)
      fmt.Println(string(result))
    }
    ```

### Example response

```json
{
  "id": "1",
  "jsonrpc": "2.0",
  "result": "5898"
}
```

### Arguments

| Parameter | Type   | Required | Description           |
| --------- | ------ | -------- | --------------------- |
| `id`      | string | Required | `"1"`                 |
| `jsonrpc` | string | Required | `"2.0"`               |
| `method`  | string | Required | `"GetCurrentDSEpoch"` |
| `params`  | string | Required | Empty string `""`     |
