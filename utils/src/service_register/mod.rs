//服务集合
pub type Services = HashMap<String, Service>;

//服务
pub struct Service {
    pub id: String,
    pub service: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub datacenter: String,
}

//注册所需数据
pub struct Registration {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub tags: Vec<String>,
    pub check: Option<GrpcHealthCheck>,
}

pub struct GrpcHealthCheck {
    pub name: String,
    pub grpc: String,
    pub grpc_use_tls: bool,
    pub interval: String,
}

pub trait ServiceRegister: Send + Sync + Debug {
    /// 服务注册
    async fn register(&self, registration: Registration) -> Result<(), Error>;

    /// 服务发现
    async fn discovery(&self) -> Result<Services, Error>;

    /// 服务注销
    async fn deregister(&self, service_id: &str) -> Result<(), Error>;

    /// 服务筛选
    async fn filter_by_name(&self, name: &str) -> Result<Services, Error>;
}
