use yew::{
    prelude::*,
};


pub enum Msg {
}

pub struct Tutorial {
}

impl Component for Tutorial {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {

        Self {
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {

            <div class="base-landing">

                <div class="container">  
                    <h1 style="align-items: center; text-align:center;margin-bottom: -90px; margin-top: 100px">{"Application Tutorial"}</h1>
                    <p style="align-items: center; text-align:center; margin-bottom: -120px; margin-top: 100px">{"In order to use the app, make sure to register an account and login first"}</p>
                </div> 

                <div class="landing-page">
                    <div class="container">  
                            <h3 style="align-items: center; text-align:center; margin-top: 40px"><b>{"Initial Setup"}</b></h3>
                                <h5 style="margin-top: 30px"><b>{"Connect Jira Notification to Your Account"}</b></h5>
                                    <ol type="1">
                                        <li>{"Go to Profile page"}</li>
                                        <li style="margin-top: 5px">{"Click the \"Create Webhook\" button"}</li>
                                        <li style="margin-top: 5px">{"Fill in the required data and submit"}</li>
                                        <li style="margin-top: 5px" align="justify">{"A success message will pop up if webhook is successfully created and data in Profile page will be updated"}</li>
                                    </ol> 

                                <h5 style="margin-top: 50px"><b>{"Making Sure the Webhook is Set Correctly"}</b></h5>
                                    <ol type="1">
                                        <li>{"Click the \"Check Webhook\" button in the updated Profile page"}</li>
                                        <li style="margin-top: 5px" align="justify">{"The \"Webhook Status\" field in Profile page will be updated based on the checking result"}</li>
                                        <li style="margin-top: 5px" align="justify">{"If the status is \"Nonfunctional\" you won't be able to receive notification, click the \"Repair Webhook\" button to fix the webhook and make sure the status is \"Functional\""}</li>
                                    </ol> 
                    </div>

                <div class="container">
                        <div class="image">
                            <img src="img/tes3.png" width="600px" height="550px"/>
                        </div>
                    </div>                   
                </div>

                <div class="container" style="margin-top: -20px">                        
                        <div class="clearfix"></div>
                        <h3 style="align-items: center; text-align:center"><b>{"Connector Setup"}</b></h3>
                            <h5 style="margin-top: 30px"><b>{"Creating the Connector"}</b></h5>
                                <ol type="1">
                                    <li>{"Click the \"Create Connector\" button in the Connector Home page"}</li>
                                    <li style="margin-top: 5px">{"Fill in the required data and click the \"Create New Connector\" button"}</li>
                                    <li style="margin-top: 5px">{"The connector will show up in the Connector Home page if it's successfully created"}</li>
                                </ol> 
                    

                            <h5 style="margin-top: 50px"><b>{"Connecting to Telegram and Activating the Connector"}</b></h5>
                                <ol type="1">
                                    <li>{"Click the connector in the Connector Home page"}</li>
                                    <li style="margin-top: 5px" align="justify">{"Under the \"Bot Setting\" text, you can change the Telegram setting including the Telegram bot used and Telegram chat destination (more info can be seen by clicking the '?' icon beside the Telegram data field)"}</li>
                                    <li style="margin-top: 5px">{"You can select Jira projects that you want to be notified with by clicking the \"Select Project\" button and choosing the prefered projects' checkboxes"}</li>
                                    <li style="margin-top: 5px">{"You can select Jira events that you want to be notified with by choosing the checkboxes under the \"Event to Notify\" text"}</li>
                                    <li style="margin-top: 5px" align="justify">{"Schedule connector feature, in which the notification sent to Telegram only happens in the specified duration.
                                         Click the \"On/Off\" button under the \"Schedule\" text and fill in the preferred schedule duration"}</li>
                                    <li style="margin-top: 5px">{"You can see the connector's notification log by clicking the \"View Log\" button"}</li>
                                    <li style="margin-top: 5px">{"To activate the connector, click the \"Active/Deactive\" button beside the \"View Log\" button"}</li>
                                    <li style="margin-top: 5px">{"Click the \"Save\" button to keep all the changes made on the connector, otherwise all the previous changes won't be saved"}</li>
                                    <li style="margin-top: 5px">{"If a success message popped up, your Jira notification is now connected to Telegram"}</li>
                                </ol>  
                    </div>
            </div>
        }
    }
}