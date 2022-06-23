CREATE TABLE `aurach_user` (
   `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'unique ID',
   `name` varchar(64) NOT NULL COMMENT '用户名',
   `status` tinyint(4) default 1 NOT NULL COMMENT '状态',
   `create_time` timestamp default CURRENT_TIMESTAMP not null comment '创建时间',
   `update_time` timestamp default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新时间',
   PRIMARY KEY (`id`),
   INDEX `idx_status` (`status`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='aurach user表';

CREATE TABLE `aurach_task_info` (
   `id` bigint NOT NULL AUTO_INCREMENT COMMENT 'unique ID',
   `name` varchar(64) NOT NULL COMMENT '任务名',
   `status` tinyint(4) default 1 NOT NULL COMMENT '状态',
   `create_time` timestamp default CURRENT_TIMESTAMP not null comment '创建时间',
   `update_time` timestamp default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新时间',
   PRIMARY KEY (`id`),
   INDEX `idx_status` (`status`)
) ENGINE=InnoDB AUTO_INCREMENT=10 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci COMMENT='aurach task info表';