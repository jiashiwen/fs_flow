# ToDo

## 技术验证

- [x] 验证tokio multy threads 在执行时是否阻塞

## 功能列表

### 0.2.0
- [x] 工程整理，升级依赖，剔除不必要的依赖和代码
- [x] 实现基础的jd s3 client
- [x] 实现基础的 阿里云 s3 client
- [x] 实现 aws client
- [x] 抽象各个厂商的oss client 形成统一的产生函数
- [x] 抽象ossaction trait
- [x] 实现读取和上传object bytes
- [x] jrss适配，相关文档<http://jrss-portal-public.jdfmgt.com/>
- [x] 多线程验证rayon 和 tokio两个方案
- [x] 编写多线程程序
- [x] checkpoint 设计与实现 transfer
- [x] checkpoint 设计与实现 download
- [x] checkpoint 设计与实现 upload
- [x] 最大错误数机制，到达最大错误数，任务停止
- [x] 错误数据持久化与解析，配合checkpoint实现断点续传
- [x] 支持文件过期时间
- [x] 大文件支持<https://docs.aws.amazon.com/sdk-for-rust/latest/dg/rust_s3_code_examples.html>
  - [x] 大文件分片上传
  - [x] 大文件分片下载
- [x] 增加local_to_local 任务，多线程复制目录文件
  - [x] 大文件文件流切分
  - [x] 大文件流写入
- [x] 当源端文件不存在则视作成功处理，记录offset
- [x] 支持oss文件过期时间
- [x] 静态校验功能
  - [x] target 是否存在
  - [x] 过期时间差值是否在一个范围之内
  - [x] content_length 源于目标是否一致
  - [x] 精确校验模式：通过流方式读取源与目标文件，校验buffer是否一致，如此只消耗网络流量
- [x] 支持last_modify_grater ,最后更改时间源端大于上次同步时获取object list 列表的初始时间戳,配置增量参数incremental，当该参数为真，在每次同步结束时获取本次同步起始时间戳，遍历源，生成时间戳大于该时间戳的的对象列表，执行同步任务，重复以上过程，直到object list内容为空
- [x] 进度展示
  - [x] 命令行进度选型
  - [x] 迁移总文件数统计
  - [x] 迁移完成文件数统计
  - [x] 计算总体进度百分比
  - [x] checkpoint新增total_lines 和 finished_lines
  
### 0.2.1
- [x] source analysis 功能，源端文件分析，分析大小文件所占比例，便于制定并发数量策略
- [x] upload 及 local2local 任务，通过使用inotify记录增量
- [x] 由于loop占用线程时间太长消耗系统资源，需要改进文件怎量同步部分代码，使用定时器定期轮询文件
  - [x] 定义增量记录文件，记录每次轮询的文件名及offset
  - [x] 利用 tokio::sleep 减轻cpu负载
  - [x] 定时器定时按记录文件的offset执行增量任务
- [x] 文件上传断点续传实现
  - [x] 实现机制：先开启notify记录变动文件；每次断点遍历上传文件目录，取last_modify 大于  checkpoint 记录时间戳的文件上传。需要考虑大量文件的遍历时间，需要做相关实验验证
  - [x] 兼容minio
    - [x] minio 环境部署
    - [x] 适配minio
  - [x] 为便于扩展，新增objectstorage enum，用来区分不同数据源
  - [x] 解决modify create modify重复问题
  - [x] 增量逻辑新增正则过滤
  - [x] 将oss 增量代码归并至oss client
  - [x] 支持从指定时间戳同步，通过时间戳筛选大于或小于该时间戳的文件
- [x] 升级aws至最新版本 
- [ ] 多任务模式，统一描述文件支持多任务同事运行
- [ ] 支持oss存储类型，通过 set_storage_class 实现
- [x] 增加模板输出功能，增加文件名参数。
- [ ] 新增precheck，并给出校验清单
  - [ ] 归档文件不能访问需提前校验
- [ ] 编写makefile 实现交叉编译
- [ ] 设计多任务snapshot，统一管理任务状态
  - [ ] 任务相关状态字段定义，任务checkpoint，任务错误数，任务offset map，停止标识
- [ ] 适配七牛
  - [ ] prefix map,将源prefix 映射到目标的另一个prefix
  - [ ] 全量、存量、增量分处理
  - [ ] 修改源为oss的同步机制，base taget 计算removed 和 modified objects
- [ ] s3 multipart upload 断点续传，多快上传时，能够从未上传块开始上传 
- [ ] 验证 &mut JoinSet<()>, Arc方式，主要验证是否可向下传递
- [ ] oss2oss multi thread upload part 改造
- [ ] 日志优化，每条记录输出任务id；规范info 输出格式，包括，任务id，msg，输出类型等
- [ ] 使用rocksdb进行状态管理，meta_data 实现自管理
- [ ] 多任务管理
- [ ] 验证global tokio::runtime,用于任务运行空间
- [ ] 任务限速

### split object list
- [x] 编写object list 文件拆分相关函数，包括从oss获取 object，以及从文件系统获取相关的文件列表
- [x] 新增 gen_obj_list_multi_files 命令行功能，生成meta_dir下多个文件列表以及顺序列表和对应的第一个checkpoint
- [] 变更checkpoint记录机制，根据record 中的文件号记录执行文件，在记录时重新获取记录文件的byts以及行数
- [] 新增 task exec 参数 --start_from_checkpoint 便于在执行时通过参数影响任务行为
- [x] 变更代码，使用sdk提供的 .customize().disable_payload_signing().send() 替换.send_with_plugins(presigning)
- [ ] 将到本地的文件路径进行转换 将'//','///' 等转换为标准路径，若key开头为'/',拼接后进行标准化路径工作
- [ ] oss2local路径归一化转换参数，将源路径归一化
- [ ] 增加oss2oss 路径归一化转换参数，将源路径归一化
- [x] template 变更，增加过滤包含http格式的key \b[\w-]*(https?|ftp|file):\/\/\S+
- [x] 重构流程，通过sequenc list 遍历object list 文件构造record；读取checkpoint时按照file_num 指定正在执行的文件
- [x] 调整checkpoint 存储规则，以适应列表多文件
- [x] 将所有任务中的函数变更为异步函数，runtime 在command中指定
- [x] compare 模块改造，适应多文件列表
- [ ] compare 模型增加，按列表校验功能
- [ ] compare 自己长度时使用head_object 减小网络通信
- [ ] 增量模式参数 IncrementMode ，notify 和 scan scan模式下interval 参数指定轮训时间
- [x] 在任务开始之前，清理target上传分片
- [ ] 新增error collect功能用于将错误日志归集为object list
- [x] 根据列表迁移：源端文件列表，每一行一个对象路径，遍历文件执行同步操作
- [x] 限流方案测试
      ```
      [dependencies]
      governor = "0.6"
      ```
      ```
      use governor::{Quota, RateLimiter};
      use std::num::NonZeroU32;
      
      // 创建限制器（例如 1MB/s）
      let byte_limit = NonZeroU32::new(1_000_000).unwrap();
      let quota = Quota::per_second(byte_limit);
      let limiter = RateLimiter::direct(quota);
      
      // 在发送数据前等待许可
      async fn send_data(data: &[u8]) {
          limiter.until_n_ready(data.len() as u32).await;
          // 实际发送数据...
      }
      ```
      结论，该方案只能限制强求频率无法限制流速

- [x] 验证trickle 限流方案 方案不可用
- [ ] 增量scan方式把interval暴漏为参数
- [x] 优化changed_object_capture_based_targe函数，拆解步骤，新增 capture_removed_objects_to_file、capture_modified_objects_to_file，为后续并行处理做准备



q3
- [ ] 进一步优化start_tranfer 函数，将代码进一步拆分为 execute_stock_transfer、execute_incremental_transfer 两个函数，分别执行存量和增量迁移任务
- [ ] 优化changed_object_capture_based_targe 并发 capture_removed_objects 和 capture_modified_objects
- [ ] 新增本地新增scan模式，应对共享存储notify失效问题
- [ ] 每个模块新增utils.rs，用于管理模块内的公共函数
- [ ] 将工程model集中管理，按各个模块命名，充分共享struct
  - [ ] 归集task、task_transfer、task_compare、task_delete
  - [ ] 归集s3、checkpoint、filter
  - [ ] 归集 transfer_executors、compare_executors
- [x] 常量统一管理
- [ ] 多平台编译
- [] 调研-- rust 如何使用多个版本的依赖
- [x] checkpoint 增加last_scan_timestamp 字段，记录上次扫描的时间戳，用于增量扫描时过滤已扫描的对象
  - [x] 修复 checkpoint struct
  - [ ] 修改 增量对应代码
- [ ]优化 task执行流程，start_transfer 重构
  - [ ] 增加init阶段，初始化任务，检查任务状态，恢复任务进度，创建任务检查点文件
  - [ ] 增加stock阶段，存量数据迁移，迁移存量数据，迁移完成后，更新任务检查点文件
  - [ ] 增加increment阶段，增量数据迁移，迁移增量数据，迁移完成后，更新任务检查点文件
- [ ] task analyze 并发改造

  



## 文档
- [ ] 测试方案 -- 详细测试项及测试流程
- [ ] 设计文档 -- 流程及机制描述

## 校验项

- [x]last_modify 时间戳，目标大于源

## 错误处理及断点续传机制

- 日志及执行offset记录
  - 执行任务时每线程offset日志后写，记录完成的记录在文件中的offset，offset文件名为 文件名前缀+第一条记录的offset
  - 某一记录执行出错时记录record及offset，当错误数达到某一阀值，停止任务
- 断点续传机制
  - 先根据错误记录日志做补偿同步，检查源端对象是否存在，若不存在则跳过
  - 通过offset日志找到每个文件中的最大值，并所有文件的最大值取最小值，再与checkpoint中的offset比较取最小值，作为objectlist的起始offset。

## 多个object list 文件，限制object list 文件的最大值，便于执行文件多的buckt
## 增量实现

### 源为oss时的机制

- 回源实现方式
  - 完成一次全量同步
  - 配置对象存储回源
  - 应用切换到目标对象存储
  - 再次发起同步，只同步目标不存在或时间戳大于目标对象的对象

- 逼近法实现近似增量
  - 应用程序设置为非删除操作
  - 完成一次全量同步
  - 二次同步，只同步新增和修改(时间戳大于目标的)，并记录更新数量
  - 知道新增更新在一个可接接受的范围，切换应用
  - 再次同步，只同步新增和修改(时间戳大于目标的)

### 源为本地文件系统（linux）

通过 inotify 获取目录变更事件（需要验证 <https://crates.io/crates/notify）>

## 面临问题 -- sdk 兼容性问题，需要实验

- [x] 如何获取object属性
- [x] 如何判断object是否存在
- [x] 文件追加和覆盖哪个效率更高待验证

## 测试验证

- [x] tokio  实现多线程稳定性，通过循环多线程upload测试,异步情况需重新考虑trait。
- [x] 验证 从oss流式读取行，内存直接put 到oss
- [ ] 验证 crossbeam_utils::sync::WaitGroup 在 async 下是否起作用

## 多线程任务

- [x] 多线程任务模型设计

## 服务化改造

- [ ] 服务化架构设计
- [ ] 任务meta data设计

## 分布式改造

## 日志

11月9
改进增量断点续传方案，抓取target 的 remove 和  modify，执行完成后，执行增量逻辑
12月1
完成校验框架


问题是由于 request style 引发，变更为path style 问题解决

The problem is caused by request style, change to path style to solve the problem

```rust
let s3_config_builder = aws_sdk_s3::config::Builder::from(&config).force_path_style(true);
let client = aws_sdk_s3::Client::from_conf(s3_config_builder.build());
```

thanks!

- 2025-04-11
  - oss key 扫描存储多文件
  - 本地文件扫描存储多文件
  - checkpoint 结构变革
- 2025-04-18
  - 开发多文件列表执行传输任务的执行器
  - 变更checkpoint记录机制，适应多文件执行中断点续传功能
  - 重构代码，compare、transfer构建独立模块
- 2025-04-25
  - 变更代码：移除send_with_plugins(presigning),变更为    .customize().disable_payload_signing().send()，使用aws sdk 标准函数
  - 使用 onecell 替换 lazy static
  - 新增config，控制日志输出level
- 2025-05-06
  - oss pipe 
  - compare 模块改造，适应多文件列表
  - 新增transfer任务开始前进行未完成的multi part 清理
  - 将所有任务中的函数变更为异步函数，runtime 在command中指定，已完成annalyze 、list_objects
- 2025-05-06
  - oss pipe 
  - compare 模块改造，适应多文件列表
  - 新增transfer任务开始前进行未完成的multi part 清理
  - 将所有任务中的函数变更为异步函数，runtime 在command中指定，已完成annalyze 、list_objects

- 2025-05-28
  - 在任务开始之前，清理target上传分片
  - 增加trubleshooting 文档
  - 完成根据文件列表迁移
  
- 2025-06-4
- 验证trickle 限流方案 方案不可用
- governor qp限流，不符合要求


bug修复：
compare模块由于未阻塞线程，导致任务提前结束，部分compare任务未执行，bug已修复

效能科技asr服务交流
  
- 新增pre_check 模块用于在任务开始时校验源于不低端的可用性
- 变更compare expr 模块，使用expires_string 替换expires(),适配新版sdk
- 优化changed_object_capture_based_targe函数，拆解步骤，新增 capture_removed_objects_to_file、capture_modified_objects_to_file，为后续并行处理做准备；
- 新增 TransferTask.attributes increment_mode 字段，用于指定增量模式，目前支持 notify 和 scan
- 新增 TransferTask.attributes interval 字段，用于指定增量模式下的轮训时间


1.架构优化
为便于元数据管理、本期改造对记录传输对象的列表文件进行了拆分工作，原有单一记录文件变更为多文件记录，以保证在海量对象传输过程中单文件的传输效率问题。工程相关变更如下：
- 编写object list 文件拆分相关函数，包括从oss获取 object，以及从文件系统获取相关的文件列表
- 调整checkpoint 存储规则，以适应列表多文件
- 新增 gen_obj_list_multi_files 命令行功能，生成meta_dir下多个文件列表以及顺序列表和对应的第一个checkpoint
- compare 模块改造，适应多文件列表
- 重构流程，通过sequenc list 遍历object list 文件构造record；读取checkpoint时按照file_num 指定正在执行的文件

2.功能优化
- 新增按列表传输功能，用户可以自行编辑传输对象列表文件
- 在任务开始之前，清理target上传分片
- 将所有任务中的函数变更为异步函数，runtime 在command中指定

3.工程优化
- 优化changed_object_capture_based_targe函数，拆解步骤，新增 capture_removed_objects_to_file、capture_modified_objects_to_file，为后续并行处理做准备
- 变更代码，使用sdk提供的 .customize().disable_payload_signing().send() 替换.send_with_plugins(presigning)








