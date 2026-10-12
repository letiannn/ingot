use serde::Serialize;

/// Supported target platforms (OS/RTOS-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Target {
    Baremetal,
    FreeRtos,
    RtThread,
    Linux,
}

/// Target-specific configuration for code generation.
#[derive(Debug, Clone, Serialize)]
pub struct TargetConfig {
    pub target: Target,
    pub pointer_width: u8,
    pub alignment: u8,
    pub max_inline_size: usize,

    /// Whether this target supports mutex-based thread safety.
    pub has_mutex: bool,
    /// C #include lines for mutex support (empty if no mutex).
    pub mutex_include: String,
    /// C type for the mutex variable.
    pub mutex_type: String,
    /// C statement to declare+init the mutex (static initializer or empty).
    pub mutex_decl: String,
    /// C statement to initialize mutex at runtime (in DataModel_Initialize).
    pub mutex_init: String,
    /// C statement to destroy mutex (in DataModel_TearDown).
    pub mutex_destroy: String,
    /// C expression to lock the mutex.
    pub mutex_lock: String,
    /// C expression to unlock the mutex.
    pub mutex_unlock: String,
}

impl TargetConfig {
    pub fn for_target(target: Target) -> Self {
        match target {
            Target::Baremetal => Self {
                target,
                pointer_width: 32,
                alignment: 4,
                max_inline_size: 64,
                has_mutex: false,
                mutex_include: String::new(),
                mutex_type: String::new(),
                mutex_decl: String::new(),
                mutex_init: String::new(),
                mutex_destroy: String::new(),
                mutex_lock: String::new(),
                mutex_unlock: String::new(),
            },
            Target::FreeRtos => Self {
                target,
                pointer_width: 32,
                alignment: 4,
                max_inline_size: 64,
                has_mutex: true,
                mutex_include: "#include \"freertos/FreeRTOS.h\"\n#include \"freertos/semphr.h\""
                    .to_string(),
                mutex_type: "SemaphoreHandle_t".to_string(),
                mutex_decl: "static SemaphoreHandle_t dm_mutex = NULL;".to_string(),
                mutex_init: "dm_mutex = xSemaphoreCreateMutex();".to_string(),
                mutex_destroy:
                    "if (dm_mutex != NULL) { vSemaphoreDelete(dm_mutex); dm_mutex = NULL; }"
                        .to_string(),
                mutex_lock: "xSemaphoreTake(dm_mutex, portMAX_DELAY)".to_string(),
                mutex_unlock: "xSemaphoreGive(dm_mutex)".to_string(),
            },
            Target::RtThread => Self {
                target,
                pointer_width: 32,
                alignment: 4,
                max_inline_size: 64,
                has_mutex: true,
                mutex_include: "#include <rtthread.h>".to_string(),
                mutex_type: "rt_mutex_t".to_string(),
                mutex_decl: "static rt_mutex_t dm_mutex = RT_NULL;".to_string(),
                mutex_init:
                    "dm_mutex = rt_mutex_create(\"dm_mutex\", RT_IPC_FLAG_FIFO);".to_string(),
                mutex_destroy:
                    "if (dm_mutex != RT_NULL) { rt_mutex_delete(dm_mutex); dm_mutex = RT_NULL; }"
                        .to_string(),
                mutex_lock: "rt_mutex_take(dm_mutex, RT_WAITING_FOREVER)".to_string(),
                mutex_unlock: "rt_mutex_release(dm_mutex)".to_string(),
            },
            Target::Linux => Self {
                target,
                pointer_width: 64,
                alignment: 8,
                max_inline_size: 128,
                has_mutex: true,
                mutex_include: "#include <pthread.h>".to_string(),
                mutex_type: "pthread_mutex_t".to_string(),
                mutex_decl: "static pthread_mutex_t dm_mutex = PTHREAD_MUTEX_INITIALIZER;"
                    .to_string(),
                mutex_init: String::new(),
                mutex_destroy: "pthread_mutex_destroy(&dm_mutex);".to_string(),
                mutex_lock: "pthread_mutex_lock(&dm_mutex)".to_string(),
                mutex_unlock: "pthread_mutex_unlock(&dm_mutex)".to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_has_pthread_mutex() {
        let cfg = TargetConfig::for_target(Target::Linux);
        assert!(cfg.has_mutex);
        assert!(cfg.mutex_include.contains("pthread.h"));
        assert!(cfg.mutex_lock.contains("pthread_mutex_lock"));
    }

    #[test]
    fn freertos_has_freertos_mutex() {
        let cfg = TargetConfig::for_target(Target::FreeRtos);
        assert!(cfg.has_mutex);
        assert!(cfg.mutex_include.contains("semphr.h"));
        assert!(cfg.mutex_lock.contains("xSemaphoreTake"));
    }

    #[test]
    fn rtthread_has_rtthread_mutex() {
        let cfg = TargetConfig::for_target(Target::RtThread);
        assert!(cfg.has_mutex);
        assert!(cfg.mutex_include.contains("rtthread.h"));
        assert!(cfg.mutex_lock.contains("rt_mutex_take"));
    }

    #[test]
    fn baremetal_has_no_mutex() {
        let cfg = TargetConfig::for_target(Target::Baremetal);
        assert!(!cfg.has_mutex);
        assert!(cfg.mutex_lock.is_empty());
        assert_eq!(cfg.pointer_width, 32);
        assert_eq!(cfg.alignment, 4);
    }
}