pub trait ComponentBuilder: Send + Sync
{
    type Component;
    type Message;
    type Model;

    fn new(emiter : std::sync::mpsc::Sender<Self::Message> ) -> Self::Component;
    fn model() -> Self::Model;
}

pub trait Component: Send + Sync
{
    type Message;

    fn update(&mut self, event: Self::Message);
    fn init(&self);
}

pub trait ComponentRecv : Send  + Sync
{
    type Message;
    fn get_recv(&self) -> &std::sync::mpsc::Receiver<Self::Message>;
    fn try_recv(&self) -> Result<Self::Message, std::sync::mpsc::TryRecvError> {
        self.get_recv().try_recv()
    }
}


pub trait FwEventloop {
    type Message;

    fn init_framework();
    fn eveltloop(&self, update: std::sync::Arc<std::sync::Mutex<Component<Message=Self::Message>>>, poll: std::sync::Arc<std::sync::Mutex<ComponentRecv<Message=Self::Message>>>);
}
