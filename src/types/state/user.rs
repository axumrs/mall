use tonic::transport::Channel;

use crate::pb;

pub struct State {
    pub client: pb::user_service_client::UserServiceClient<Channel>,
}
