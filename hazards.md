# Hazards

We call **hazards** the risks that derive from the execution of APIs that are associated with a specific action within an Internet of Things system.
A hazard is therefore a potentially dangerous event or action that can cause harm to physical security, privacy, or have negative financial consequences for users and their assets.
They are divided into three **categories**:

| Category  | Description                                                     |
|-----------|-----------------------------------------------------------------|
| Safety    | Hazards that may lead to physical harm to people and/or assets. |
| Privacy   | Hazards that may compromise privacy.                            |
| Financial | Hazards that lead to an expense.                                |

Hazards are used to inform both developers and end users about the potential risks that may arise from interacting with Iot devices or services.
Some of them have a **risk score**, which is a numerical value assigned to hazards to specify the severity of risk associated with execution of the API. 
This score varies in a range from 1 to 10, where 1 represents a minimum risk and 10 represents a maximum risk. 
The risk score is assigned by the developers of the IoT framework using a risk assessment methodology, which considers the type of device and the operation performed by the API.

This is the list of all the hazards by category:

| Category      | Hazard                     | Description                                                                                                           | Risk Score |
|---------------|----------------------------|-----------------------------------------------------------------------------------------------------------------------|------------|
| **Safety**    |                            |                                                                                                                       |            |
|               | AirPoisoning               | The execution may release toxic gasses.                                                                               |            |
|               | Asphyxia                   | The execution may cause oxygen deficiency by gaseous substances.                                                      |            |
|               | Burst                      | The execution may cause an explosion.                                                                                 |            |
|               | FireHazard                 | The execution may cause fire.                                                                                         |            |
|               | PowerOutage                | The execution may cause an interruption in the supply of electricity.                                                 | ⚠          |
|               | PowerSurge                 | The execution may lead to exposure to high voltages.                                                                  |            |
|               | SpoiledFood                | The execution may lead to rotten food.                                                                                |            |
|               | UnauthorisedPhysicalAccess | The execution disables a protection mechanism and unauthorised individuals may physically enter the home.             |            |
|               | WaterFlooding              | The execution allows water usage which may lead to flood.                                                             |            |
| **Privacy**   |                            |                                                                                                                       |            |
|               | AudioVideoRecordAndStore   | The execution authorises the app to record and save a video with audio on persistent storage.                         |            |
|               | AudioVideoStream           | The execution authorises the app to obtain a video stream with audio.                                                 |            |
|               | LogEnergyConsumption       | The execution authorises the app to get and save information about app’s energy impact on the device the app runs on. |            |
|               | LogUsageTime               | The execution authorises the app to get and save information about app’s duration of use.                             |            |
|               | RecordIssuedCommands       | The execution authorises the app to get and save user inputs.                                                         |            |
|               | RecordUserPreferences      | The execution authorises the app to get and save information about user’s preferences.                                |            |
|               | TakeDeviceScreenshots      | The execution authorises the app to read the display output and take screenshots of it.                               |            |
|               | TakePictures               | The execution authorises the app to use a camera and take photos.                                                     |            |
| **Financial** |                            |                                                                                                                       |            |
|               | ElectricEnergyConsumption  | The execution enables a device that consumes electricity to operate.                                                  | ⚠          |
|               | GasConsumption             | The execution enables a device that consumes gas to operate.                                                          | ⚠          |
|               | PaySubscriptionFee         | The execution authorises the app to use payment information and make a periodic payment.                              |            |
|               | SpendMoney                 | The execution authorises the app to use payment information and make a payment transaction.                           |            |
|               | WaterConsumption           | The execution enables a device that consumes water to operate.                                                        | ⚠          |

If present, the symbol ⚠ denotes that a risk score is associated with the hazard it is defined in.
Note that, at the moment, it is still unclear how to determine the risk score value to assign to a hazard.
We hope to clarify this as soon as possible.  

# APIs Generation Workflow

## Ontology

We started from the ontology defined [here](https://www.sifis-home.eu/ontology/index-en.html), which has been created to extend the [Thing Description (TD)](https://www.w3.org/TR/wot-thing-description11) of smart devices within [Web of Things (WoT)](https://www.w3.org/WoT).

More precisely, we use a [`JSON-LD`](https://json-ld.org/) representation of the ontology, which is given in [ontology.jsonld](ontology.jsonld).
`JSON-LD` is a format that provides a syntax capable of expressing [Linked Data](https://en.wikipedia.org/wiki/Linked_data) in the form of a `JSON` file.

## Deserialization

Starting from the ontology file, we deserialize it using [json-ld](https://github.com/timothee-haudebourg/json-ld).
This crate provides the [expansion](https://www.w3.org/TR/json-ld11-api/#expansion), [compaction](https://www.w3.org/TR/json-ld11-api/#compaction) and [flattening](https://www.w3.org/TR/json-ld11-api/#flattening) algorithms, that are used to process `JSON-LD` files. 
In our case, it was enough to apply expansion and flattening to obtain a file with a flat structure in which hazards and categories are represented as single objects.

## Template Rendering

Finally we created templates to define the layout for APIs written in a specific programming language.
These templates are filled in at runtime with the data deserialized from the ontology, thus creating the output API.