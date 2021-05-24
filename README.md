
# AppConfig API

AppConfig Service (Analog AWS AppConfig)

## Indices

* [Get Config](#1-get-config)
* [Ping](#2-ping)

--------

### 1. Get Config

Get configuration from service


***Endpoint:***

```HTTP
Method: GET
URL: /applications/{{Application}}/environments/{{Environment}}/configurations/{{Configuration}}?client_configuration_version={{ClientConfigurationVersion}}&client_id={{ClientId}} HTTP/1.1
```


***More example Requests/Responses:***


##### I. Example Response: 500 Internal Server Error
```js
Error parse Yaml!!
```

***Status Code:*** 500

<br>



##### II. Example Response: 200 OK
```yml
---
experiments: {}
preRegistrationExperiments: {}
limits:
  unsealedSenderNumber:
    maxCardinality: 100
    ttl: PT24H
    ttlJitter: PT24H
  unsealedSenderIp:
    bucketSize: 120
    leakRatePerMinute: 0.03333333333333333
remoteDeprecation:
  minimumVersions: {}
  versionsPendingDeprecation: {}
  blockedVersions: {}
  versionsPendingBlock: {}
  unrecognizedUserAgentAllowed: true
messageRate:
  enforceUnsealedSenderRateLimit: false
  rateLimitedCountryCodes: []
  rateLimitedHosts: []
  responseDelay: 0.0012
  responseDelayJitter: 0.0005
  receiptDelay: 1.2
  receiptDelayJitter: 0.8
  receiptProbability: 0.82
payments:
  allowedCountryCodes: []
featureFlags: []
twilio:
  numbers: []
signupCaptcha:
  countryCodes: []

```

***Status Code:*** 200

<br>


##### III. Example Response: 404 Not Found
```js
Config not found!!
```

***Status Code:*** 404

<br>



### 2. Ping


Ping Service


***Endpoint:***

```bash
Method: GET
URL: /ping
```



***More example Requests/Responses:***

##### I. Example Response: 200 OK
```js
pong - AppConfig service
```

***Status Code:*** 200

<br>


---
[Back to top](#AppConfig-API)
