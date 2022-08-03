# redissyncer-cli-rs

[中文](README_cn.md)

[redissyncer](https://github.com/TraceNature/redissyncer-server)
command line interface for migrate task。

## Build

[build document](https://github.com/TraceNature/redissyncer-cli-rs/blob/main/docs/build.md)

## Function and usage

* redissyncer-cli-rs supports command line mode and interactive mode, "redissyncer-cli -i" enters interactive mode
* This client program is used by the redissyncer-server client program to create, start, stop, and monitor redis
  synchronization tasks. Before using this client, please ensure that the server program is running normally

## Operation guide

* Generate default configuration file

```shell
redissyncer-cli-rs config gendefault
mv config_default.yml config.yml
```

* Enter interactive mode

```shell
redissyncer-cli-rs -i
```

* Configure the redissyncer-server address

```shell
redissyncer-cli-rs> server setting http://127.0.0.1:8080
```

* login

```shell
redissyncer-cli-rs> login <username> <password>
```

redissyncer-server Default username and password: admin 123456

* Create task

  Create a task from a json file createtask.json file

  ```json
  {
  "dbNum": {
    "1": "1"
  },
  "sourcePassword": "xxxxxx",
  "sourceRedisAddress": "10.0.1.100:6379",
  "targetRedisAddress": "192.168.0.100:6379",
  "targetPassword": "xxxxxx",
  "targetRedisVersion": 4,
  "taskName": "testtask",
  "autostart": true,
  "afresh": true,
  "batchSize": 100
  }

  ```

  ```shell
  redissyncer-cli-rs> task create source ./createtask.json;
  ```

For more task modes, please refer
to [Task Template](https://github.com/TraceNature/redissyncer-cli-rs/tree/main/docs/taskjsonexample)

* List task information
    * List all tasks

      ```shell
      redissyncer-cli-rs> task list all
      ```

    * View task status by task id

      ```shell
      redissyncer-cli-rs> task list bytaskid 690DEF6222E34443884033B860CE01EC
      ```

    * View task status by task name

      ```shell
      redissyncer-cli-rs> task list bynames $taskname
      ```


* Start task

   ```shell
   redissyncer-cli-rs> task start 690DEF6222E34443884033B860CE01EC
   ```

* Stop task

   ```shell
   redissyncer-cli-rs> task stop 690DEF6222E34443884033B860CE01EC
   ```

* Delete task by task ID

   ```shell
   redissyncer-cli-rs> task remove 690DEF6222E34443884033B860CE01EC
   ```
