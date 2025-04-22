# Data Collection Source File
This contains data collection json files exported from flow used for data collection.

# How to Use
## `main_red_flows.json`
* This is file contains main logic of collect and process data.

- replace `<ROBOFLOW_API_KEY>` with Roboflow's apikey by [following this guide](https://docs.roboflow.com/api-reference/authentication#retrieve-an-api-key)

- replace `<<TOMTOM_API_KEY>>` with Tom Tom's apikey follow [this guide](https://developer.tomtom.com/knowledgebase/platform/articles/how-to-get-an-tomtom-api-key/)

- replace `<<OPEN_WEATHER_API_KEY>>` with [Open Weather API](https://openweathermap.org/current) Key

- replace `<<DBHOST>>` with your mySQLDatabase Host Name

- replace `<<DBName>>` with your mySQLDatabase Database Name

- replace `<<MQTT_Broker>>` with your MQTT Broker URL.

- replace `<<MQTT_Topic>>` with your MQTT Topic.

- Import this file to **[Node Red](https://nodered.org)**.

## `mobile_red_flows.json`
* This flow is to capture the picture and send it up to MQTT Server.

- replace `<<MQTT_Broker>>` with your MQTT Broker URL this must be the same as main flow.

- replace `<<MQTT_Topic>>` with your MQTT Topic this must be the same as main flow.

- Import this file to **[Red Mobile (paid)](https://play.google.com/store/apps/details?id=com.okhiroyuki.redmobile&hl=en-US&pli=1)**
