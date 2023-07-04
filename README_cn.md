# redissyncer-cli-rs

[English](README.md)

[redissyncer](https://github.com/TraceNature/redissyncer-server) 的客户端cli工具，方便迁移任务操作。

## 构建

[构建文档](https://github.com/TraceNature/redissyncer-cli-rs/blob/main/docs/build.md)

## 功能与使用方法

* redissyncer-cli-rs 支持命令行模式和交互模式，"redissyncer-cli -i"进入交互模式
* 该客户端程序为redissyncer-server客户端程序用与创建、启停、监控redis同步任务，在使用本客户端之前请确保服务端程序正常运行

## 操作指南

* 生成默认配置文件

```shell
redissyncer-cli-rs config gendefault
mv config_default.yml config.yml
```

* 进入交互模式

```shell
redissyncer-cli-rs -i
```

* 配置 redissyncer-server 服务器地址

```shell
redissyncer-cli-rs> server setting http://127.0.0.1:8080
```

* 登录

```shell
redissyncer-cli-rs> login <username> <password>
```

redissyncer-server 默认用户名和密码: admin 123456

* 创建任务

    * 通过json文件创建任务 createtask.json文件

   ```json
   {
       "dbNum":{
           "1":"1"
       },
       "sourcePassword":"xxxxxx",
       "sourceRedisAddress":"10.0.1.100:6379",
       "targetRedisAddress":"192.168.0.100:6379",
       "targetPassword":"xxxxxx",
       "targetRedisVersion":4,
       "taskName":"testtask",
       "autostart":true,
       "afresh":true,
       "batchSize":100
   }
   ```

   ```shell
   redissyncer-cli-rs> task create source ./createtask.json;
   ```

更多任务模式请参考[任务模板](https://github.com/TraceNature/redissyncer-cli-rs/tree/main/docs/taskjsonexample)

* 查看任务
    * 查看全部任务

      ```shell
      redissyncer-cli-rs> task list all
      ```

    * 通过任务id查看任务状态

      ```shell
      redissyncer-cli-rs> task list bytaskid 690DEF6222E34443884033B860CE01EC
      ```

    * 通过任务名称查看任务状态

      ```shell
      redissyncer-cli-rs> task list bynames $taskname
      ```


* 启动任务

   ```shell
   redissyncer-cli-rs> task start 690DEF6222E34443884033B860CE01EC
   ```

* 停止任务

   ```shell
   redissyncer-cli-rs> task stop 690DEF6222E34443884033B860CE01EC
   ```

* 通过任务ID删除任务

   ```shell
   redissyncer-cli-rs> task remove 690DEF6222E34443884033B860CE01EC
   ```~~
