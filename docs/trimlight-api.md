# Trimlight V2 OAuth API Documentation

## Version History

| Version | Description | Date |
|---------|-------------|------|
| 1.0.0 | Initial version | 2021-04-20 |
| 1.1.0 | Add currentEffect data to the device details response data | 2021-11-22 |
| 1.2.0 | Add group function APIs (19-23) | 2021-12-04 |
| 1.2.1 | Add the "sync a group" API (24) | 2021-12-08 |
| 1.3.0 | Add the "Notify update shadow data" API (25) | 2022-08-18 |
| 1.4.0 | 1. Add "Overlay effect" API (26)<br>2. Add "Update device datetime" API (27)<br>3. Add APIs for backing up device data (28-30)<br>4. Add "overlayEffects" and "currentDatetime" to the device details | 2024-09-26 |

## 1. Authentication

Each request needs to carry the following request headers:
```
"authorization": "<accessToken>"
"S-ClientId": "<clientId>"
"S-Timestamp": <timestamp>  // Milliseconds relative to 1970.1.1
```

### Calculating the Access Token

Follow these steps to calculate the accessToken:

1. Concatenate strings: `"Trimlight|<S-ClientId>|<S-Timestamp>"`
2. Compute the HMAC-SHA256 of the concatenated string using `clientSecret` as the key
3. Base64 encode the computed HMAC-SHA256 value to get the accessToken

**Example:**
- clientId: `tester`
- clientSecret: `test_secret`
- timestamp: `1713166849256`
- Concatenated string: `"Trimlight|tester|1713166849256"`
- Access token (base64 of HMAC-SHA256): `"z02N77XySuOwv5OSUe0vrPwprRITb656xKPulS9ooXI="`

*Note: Contact our business team to obtain your `clientId` and `clientSecret`.*

## 2. Base URL

```
https://trimlight.ledhue.com/trimlight
```

## 3. Get Device List

### Request

```http
GET /v1/oauth/resources/devices
```

### Request Body

```json
{
    "page": 1  // 10 devices per page, use 0 or null to get all devices
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success",
    "payload": {
        "total": 2,
        "current": 1,
        "data": [
            {
                "deviceId": "xxxxxxxxxxx1",
                "name": "xxxx1",
                "switchState": 0,
                "connectivity": 1,
                "state": 0,
                "fwVersionName": "1.1.1"
            },
            {
                "deviceId": "xxxxxxxxxxx2",
                "name": "xxxx2",
                "switchState": 0,
                "connectivity": 1,
                "state": 0,
                "fwVersionName": "1.1.1"
            }
        ]
    }
}
```

#### Response Code Fields

| Field | Description | Type |
|-------|-------------|------|
| code | Result code | Integer |
| desc | Result description | String |
| payload | Result payload | Object |

#### Page Fields

| Field | Description | Type |
|-------|-------------|------|
| total | Number of all devices | Integer |
| page | Current page number | Integer |
| data | Device list | Array |

#### Device Fields

| Field | Description | Type |
|-------|-------------|------|
| deviceId | Unique ID of the device | String |
| name | Device name | String |
| switchState | Device switch state:<br>0: light off<br>1: manual mode<br>2: timer mode | Integer |
| connectivity | Device connectivity state:<br>0: offline<br>1: online | Integer |
| state | Device state:<br>0: normal<br>1: upgrading | Integer |
| fwVersionName | Device firmware version name | String |

## 4. Get Device Details

### Request

```http
POST /v1/oauth/resources/device/get
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "currentDate": {
        "year": 21,    // 2021
        "month": 1,
        "day": 1,
        "weekday": 1,
        "hours": 1,
        "minutes": 1,
        "seconds": 1
    }
}
```

##### Current Date

| Field | Description | Type |
|-------|-------------|------|
| year | Current year (relative to 2000) | Integer |
| month | Current month (1-12) | Integer |
| day | Current day (1-31) | Integer |
| weekday | Current day of week:<br>1: SUNDAY<br>2: MONDAY<br>3: TUESDAY<br>4: WEDNESDAY<br>5: THURSDAY<br>6: FRIDAY<br>7: SATURDAY | Integer |
| hours | Current hour (0-23) | Integer |
| minutes | Current minutes (0-59) | Integer |
| seconds | Current seconds (0-59) | Integer |

### Response Body

```json
{
    "code": 0,
    "desc": "success",
    "payload": {
        "name": "xxxx2",
        "switchState": 0,
        "connectivity": 1,
        "state": 0,
        "colorOrder": 0,
        "ic": 0,
        "ports": [
            {
                "id": 0,
                "start": 1,
                "end": 1024
            }
        ],
        "fwVersionName": "1.1.1",
        "effects": [
            {
                "id": 0,
                "name": "New Year",
                "category": 0,
                "mode": 0,
                "speed": 100,
                "brightness": 100,
                "pixelLen": 30,
                "reverse": false
            }
        ],
        "combinedEffect": {
            "effectIds": [0, 2, 3],
            "interval": 5
        },
        "daily": [
            {
                "id": 0,
                "enable": true,
                "effectId": 0,
                "repetition": 1,
                "startTime": {
                    "hours": 10,
                    "minutes": 1
                },
                "endTime": {
                    "hours": 11,
                    "minutes": 1
                }
            }
        ],
        "calendar": [
            {
                "id": 0,
                "effectId": 1,
                "startDate": {
                    "month": 12,
                    "day": 31
                },
                "endDate": {
                    "month": 1,
                    "day": 1
                },
                "startTime": {
                    "hours": 10,
                    "minutes": 1
                },
                "endTime": {
                    "hours": 11,
                    "minutes": 1
                }
            }
        ],
        "currentEffect": {
            "category": 1,
            "mode": 1,
            "speed": 174,
            "brightness": 204,
            "pixelLen": 37,
            "reverse": false
        },
        "overlayEffects": [
            {
                "overlayType": 1,
                "targetEffect": 6
            }
        ],
        "currentDatetime": {
            "year": 24,
            "month": 9,
            "day": 26,
            "weekday": 4,
            "hours": 16,
            "minutes": 20,
            "seconds": 2
        }
    }
}
```

#### Error Response Fields

| Field | Description | Type |
|-------|-------------|------|
| code | Result code | Integer |
| desc | Result description | String |
| payload | Result payload | Object |

#### Device Fields

| Field | Description | Type |
|-------|-------------|------|
| name | Device name | String |
| switchState | Device switch state:<br>0: light off<br>1: manual mode<br>2: timer mode | Integer |
| connectivity | Device connectivity state:<br>0: offline<br>1: online | Integer |
| state | Device state:<br>0: normal<br>1: upgrading | Integer |
| colorOrder | Color order configuration | Integer |
| ic | IC type identifier | Integer |
| ports | The pixel setting for each port | List<Port> |
| fwVersionName | Device firmware version name | String |
| effects | All the effects stored in the device | List<Effect> |
| combinedEffect | Combined effect | CombinedEffect |
| daily | Daily schedules<br>Each device has two daily schedules | List<DailySchedule> |
| calendar | Calendar schedules | List<CalendarSchedule> |
| currentEffect | Device current running effect<br>(If the device's switch state is timer mode, although the light is off at this time, it will return the last running effect data.)<br>Note: The effect ID will be -1, when the controller is running a preview effect (not yet saved). | List<Effect> |
| overlayEffects | All overlay effects | List<OverlayEffect> |
| currentDatetime | The current date and time of the device | Object |

##### Effect Fields

| Field | Description | Type |
|-------|-------------|------|
| id | Effect identifier | Integer |
| category | Effect category:<br>0: Built-in<br>1: Custom | Integer |
| mode | Effect mode (0-179 for built-in, 0-16 for custom) | Integer |
| speed | Animation speed (0-255) | Integer |
| brightness | LED brightness (0-255) | Integer |
| pixelLen | Number of LEDs in effect (1-90) (only for built-in effects) | Integer |
| reverse | Reverse animation direction | Boolean |
| pixels | Custom pixel configuration (only for custom effects) | List<Pixel> |

##### Port Fields

| Field | Description | Type |
|-------|-------------|------|
| id | Port identifier | Integer |
| start | Starting LED index | Integer |
| end | Ending LED index | Integer |

##### Pixel Fields (Custom Effects)
| Field | Description | Type |
|-------|-------------|------|
| index | Pixel segment index | Integer |
| count | Number of LEDs in segment | Integer |
| color | RGB color value (hex) | Integer |
| disable | Whether segment is disabled | Boolean |

##### Combined Effect Fields

| Field | Description | Type |
|-------|-------------|------|
| effectIds | Array of effect IDs to combine | List<Integer>> |
| interval | Time between effects (minutes) | Integer |

##### Daily Schedule Fields

| Field | Description | Type |
|-------|-------------|------|
| id | Schedule identifier<br>0: daily schedule 1<br>1: daily schedule 2 | Integer |
| enable | Whether schedule is active | Boolean |
| effectId | ID of effect to display | Integer |
| repetition | Number of repetitions | Integer |
| startTime | Schedule start time | ScheduleTime |
| endTime | Schedule end time | ScheduleTime |

##### Calendar Schedule Fields

| Field | Description | Type |
|-------|-------------|------|
| id | Schedule identifier | Integer |
| effectId | ID of effect to display | Integer |
| startDate | Schedule start date | ScheduleDate |
| endDate | Schedule end date | ScheduleDate |
| startTime | Schedule start time | ScheduleTime |
| endTime | Schedule end time | ScheduleTime |

##### Schedule Time Fields

| Field | Description | Type |
|-------|-------------|------|
| hours | Schedule hours (1-24) | Integer |
| minutes | Schedule minutes (1-60) | Integer |

##### Schedule Date Fields

| Field | Description | Type |
|-------|-------------|------|
| month | Schedule month (1-12) | Integer |
| day | Schedule day of month (1-31) | Integer |


##### Overlay Effect Fields

| Field | Description | Type |
|-------|-------------|------|
| overlayType | Overylay effect type<br>0: Lightning<br>1: Snow | Integer |
| targetEffect | The target effect ID | Integer |

## 5. Set Device Switch State

### Request

```http
POST /v1/oauth/resources/device/update
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "switchState": 0  // 0=off, 1=manual, 2=timer
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 6. Set Device Name

### Request

```http
POST /v1/oauth/resources/device/update
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "name": "New Device Name"
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 7. Set Device Color Order

### Request

```http
POST /v1/oauth/resources/device/update
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "colorOrder": 0
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 8. Set Device IC

### Request

```http
POST /v1/oauth/resources/device/update
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "ic": 0
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 9. Set Device Port

### Request

```http
POST /v1/oauth/resources/device/update
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "ports": [
            {
                "id": 0,
                "start": 1,
                "end": 1024
            }
        ]
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 10. Preview Built-in Effect

### Request

```http
POST /v1/oauth/resources/device/effect/preview
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "category": 0,
        "mode": 0,
        "speed": 100,
        "brightness": 100,
        "pixelLen": 30,
        "reverse": false
    }
}
```

### Response

```json
{
    "code": 0,
    "desc": "success"
}
```

### Effect Fields

| Field | Description | Type |
|-----------|-------------|-------|
| category | Effect category (0 for built-in) | Integer |
| mode | Effect mode number (0-179) | Integer |
| speed | Animation speed (0-255) | Integer |
| brightness | LED brightness (0-255) | Integer |
| pixelLen | Number of LEDs in effect (1-90) | Integer |
| reverse | Reverse animation direction | Boolean |

## 11. Preview Custom Effect

### Request

```http
POST /v1/oauth/resources/device/effect/preview
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "category": 1,
        "mode": 0,
        "speed": 100,
        "brightness": 100,
        "pixels": [
            {
                "index": 0,
                "count": 5,
                "color": 16711680, // (0xFF0000)
                "disable": false
            }, {
                "index": 1,
                "count": 10,
                "color": 16711680,
                "disable": false
            }
        ]
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

### Effect Fields

| Field | Description | Type |
|-----------|-------------|-------|
| category | Effect category (1 for custom) | Integer |
| mode | Effect mode number (0-16) | Integer |
| speed | Animation speed (0-255) | Integer |
| brightness | LED brightness (0-255) | Integer |
| pixels | Custom effect pixels | List |

### Pixel Fields
| Field | Description | Type |
|-------|-------------|------|
| index | Pixel segment index (0-29) | Integer |
| count | Number of LEDs in segment (0-60) | Integer |
| color | RGB color decimal value (eg: 0xFF0000 => 16711680) | Integer |
| disable | Whether segment is disabled | Boolean |

## 12. Add/Update effect

### Request

```http
POST /v1/oauth/resources/device/effect/save
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "id": -1,
        "name": "xxxx",
        "category": 1,
        "mode": 0,
        "speed": 100,
        "brightness": 100,
        "pixelLen": 10,
        "reverse": false,
        "pixels": [
            {
                "index": 0,
                "count": 5,
                "color": 16711680,
                "disable": false
            },
            {
                "index": 1,
                "count": 10,
                "color": 16711680,
                "disable": false
            }
        ]
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success",
    "payload": {
      "id": 10
    }
}
```

## 13. Checkout Effect

### Request

```http
POST /v1/oauth/resources/device/effect/view
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
      "id": 10
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 14. Delete Effect

### Request

```http
POST /v1/oauth/resources/device/effect/delete
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
      "id": 10
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 15. Update daily schedule

### Request

```http
POST /v1/oauth/resources/device/daily/save
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "id": 1,
        "enable": true,
        "effectId": 1,
        "repetition": 1,
        "startTime": {
            "hours": 10,
            "minutes": 1
        },
        "endTime": {
            "hours": 11,
            "minutes": 1
        },
        "currentDate": {
            "month": 1,
            "day": 1
        }
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 16. Add/Update Calendar Schedule

### Request

```http
POST /v1/oauth/resources/device/calendar/save
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "id": 0,
        "effectId": 1,
        "startDate": {
            "month": 12,
            "day": 31
        },
        "endDate": {
            "month": 1,
            "day": 1
        },
        "startTime": {
            "hours": 10,
            "minutes": 1
        },
        "endTime": {
            "hours": 11,
            "minutes": 1
        }
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success",
    "payload": {
      "id": 0
    }
}
```

## 17. Delete Calendar Schedule

### Request

```http
POST /v1/oauth/resources/device/calendar/delete
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "id": 0
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 18. Set Combined Effect

### Request

```http
POST /v1/oauth/resources/device/combined-effect/save
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "effectIds": [0, 2, 3],
        "interval": 5
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 19. Get Group List

### Request

```http
POST /v1/oauth/resources/groups
```

### Request Body
```json
{
    "page": 1
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success",
    "payload": {
        "total": 2,
        "current": 1,
        "data": [
            {
                "groupId": "xxx",
                "name": "group1",
                "masterDevice": {
                    "deviceId": "<device_id>",
                    "name": "<device_name>"
                },
                "devices": [
                    {
                        "deviceId": "<device_id>",
                        "name": "<device_name>"
                    }
                ]
            },
            {
                "groupId": "xxx",
                "name": "group2",
                "masterDevice": {
                    "deviceId": "<device_id>",
                    "name": "<device_name>"
                },
                "devices": [
                    {
                        "deviceId": "<device_id>",
                        "name": "<device_name>"
                    }
                ]
            }
        ]
    }
}
```


## 20. Add New Group

### Request

```http
GET /v1/oauth/resources/group/add
```

### Request Body
```json
{
    "name": "<new group name>",
    "masterDevice": "<master device ID>",
    "devices": [
        "<master device ID>",
        "<device ID>",
        "<device ID>"
    ]
}
```

### Response Body

```json
{
    "deviceId": "<device-id>",
    "payload": {
        "groupId": "<new group ID>"
    }
}
```

## 21. Update Group

### Request

```http
GET /v1/oauth/resources/group/update
```

### Request Body
```json
{
    "name": "<new group name>",
    "masterDevice": "<master device ID>",
    "devices": [
        "<master device ID>",
        "<device ID>",
        "<device ID>"
    ]
}
```

### Response Body

```json
{
    "deviceId": "<device-id>",
    "payload": {
        "groupId": "<new group ID>"
    }
}
```

## 22. Rename Group

### Request

```http
GET /v1/oauth/resources/group/rename
```

### Request Body
```json
{
    "groupId": "<group ID>",
    "name": "<new group name>"
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 23. Delete Group

### Request

```http
GET /v1/oauth/resources/group/delete
```

### Request Body
```json
{
    "groupId": "<group ID>"
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 24. Sync Group

### Request

```http
GET /v1/oauth/resources/group/sync
```

### Request Body
```json
{
    "groupId": "<group ID>"
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 25. Notify Update Shadow Data

> Before requesting detailed data for the device, you can send this request to notify the device to report
the latest shadow data.

### Request

```http
GET /v1/oauth/resources/device/notify-update-shadow
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "currentDate": {
        "year": 21,
        "month": 1,
        "day": 1,
        "weekday": 1,
        "hours": 1,
        "minutes": 1,
        "seconds": 1
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 26. Update Overlay Effect

> Add specific overlay effect to effects.

### Request

```http
POST /v1/oauth/resources/device/effect/overlay
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "overlayEffects": [
            {
                "overlayType": "<overlayTypeValue>",
                "targetEffect": "<effectId>"
            },
            {
                "overlayType": "<overlayTypeValue>",
                "targetEffect": "<effectId>"
            }
        ]
    }
}
```

> If you want to remove all overlay effects, "overlayEffects" can be null or an empty list.

#### Overlay Type Fields

| Name      | Value |
|-----------|-------|
| Lightning | 0     |
| Snow      | 1     |

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 27. Update Device Datetime

### Request

```http
POST /v1/oauth/resources/device/datetime/sync
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "currentDate": {
        "year": 21,    // 2021
        "month": 1,
        "day": 1,
        "weekday": 1,
        "hours": 1,
        "minutes": 1,
        "seconds": 1
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 28. Backup Device Data

### Request

```http
POST /v1/oauth/resources/device/data/backup
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "backupTime": "yyyy-MM-dd HH:mm:ss timezone"
    }
}
```

> timezone: (HST|AKST|PST|MST|CST|EST|HDT|AKDT|PDT|MDT|CDT|EDT)

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## 29. Get Device Backup Data

### Request

```http
POST /v1/oauth/resources/device/data/backup/get
```

### Request Body
```json
{
    "deviceId": "<device-id>"
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success",
    "payload": {
        "backupKey": "xxx",
        "backupTime": "2024-09-26 16:20:26 PDT"
    }
}
```
## 30. Restore Device

### Request

```http
POST /v1/oauth/resources/device/data/backup/restore
```

### Request Body
```json
{
    "deviceId": "<device-id>",
    "payload": {
        "backupKey": "xxx"
    }
}
```

### Response Body

```json
{
    "code": 0,
    "desc": "success"
}
```

## Appendix

Here's the formatted markdown with sections and tables:

### 1. Result Code

| Code | Description |
|------|-------------|
| 0 | Success |
| 10001 | Error |
| 10002 | Wrong password |

### 2. Color Order

| Color Order | Value |
|------------|-------|
| RGB | 0 |
| RBG | 1 |
| GRB | 2 |
| GBR | 3 |
| BRG | 4 |
| BGR | 5 |

### 3. IC Type

| IC | Value |
|------------|-------|
| UCS1903 | 0 |
| DMX512 | 1 |

### 4. Built-in Effect Modes

| Name | Value |
|------|-------|
| Rainbow Gradual Chase | 0 |
| Rainbow Comet | 1 |
| Rainbow Segment | 2 |
| Rainbow Wave | 3 |
| Rainbow Meteor | 4 |
| Rainbow Gradual | 5 |
| Rainbow Jump | 6 |
| Rainbow Stars | 7 |
| Rainbow Fade In Out | 8 |
| Rainbow Spin | 9 |
| Red Stacking | 10 |
| Green Stacking | 11 |
| Blue Stacking | 12 |
| Yellow Stacking | 13 |
| Cyan Stacking | 14 |
| Purple Stacking | 15 |
| White Stacking | 16 |
| Full Color Stack | 17 |
| Red to Green Stack | 18 |
| Green to Blue Stack | 19 |
| Blue to Yellow Stack | 20 |
| Yellow to Cyan Stack | 21 |
| Cyan to Purple Stack | 22 |
| Purple to White Stack | 23 |
| Red Comet | 24 |
| Green Comet | 25 |
| Blue Comet | 26 |
| Yellow Comet | 27 |
| Cyan Comet | 28 |
| Purple Comet | 29 |
| White Comet | 30 |
| Red Meteor | 31 |
| Green Meteor | 32 |
| Blue Meteor | 33 |
| Yellow Meteor | 34 |
| Cyan Meteor | 35 |
| Purple Meteor | 36 |
| White Meteor | 37 |
| Red Wave | 38 |
| Green Wave | 39 |
| Blue Wave | 40 |
| Yellow Wave | 41 |
| Cyan Wave | 42 |
| Purple Wave | 43 |
| White Wave | 44 |
| Red Green Wave | 45 |
| Red Blue Wave | 46 |
| Red Yellow Wave | 47 |
| Red Cyan Wave | 48 |
| Red Purple Wave | 49 |
| Red White Wave | 50 |
| Green Blue Wave | 51 |
| Green Yellow Wave | 52 |
| Green Cyan Wave | 53 |
| Green Purple Wave | 54 |
| Green White Wave | 55 |
| Blue Yellow Wave | 56 |
| Blue Cyan Wave | 57 |
| Blue Purple Wave | 58 |
| Blue White Wave | 59 |
| Yellow Cyan Wave | 60 |
| Yellow Purple Wave | 61 |
| Yellow White Wave | 62 |
| Cyan Purple Wave | 63 |
| Cyan White Wave | 64 |
| Purple White Wave | 65 |
| Red Dot Pulse | 66 |
| Green Dot Pulse | 67 |
| Blue Dot Pulse | 68 |
| Yellow Dot Pulse | 69 |
| Cyan Dot Pulse | 70 |
| Purple Dot Pulse | 71 |
| White Dot Pulse | 72 |
| Red Green Blank Pulse | 73 |
| Green Blue Blank Pulse | 74 |
| Blue Yellow Blank Pulse | 75 |
| Yellow Cyan Blank Pulse | 76 |
| Cyan Purple Blank Pulse | 77 |
| Purple White Blank Pulse | 78 |
| Red with Purple Pulse | 79 |
| Green with Cyan Pulse | 80 |
| Blue with Yellow Pulse | 81 |
| Yellow with Blue Pulse | 82 |
| Cyan with Green Pulse | 83 |
| Purple with Purple Pulse | 84 |
| Red Comet Spin | 85 |
| Green Comet Spin | 86 |
| Blue Comet Spin | 87 |
| Yellow Comet Spin | 88 |
| Cyan Comet Spin | 89 |
| Purple Comet Spin | 90 |
| White Comet Spin | 91 |
| Red Dot Spin | 92 |
| Green Dot Spin | 93 |
| Blue Dot Spin | 94 |
| Yellow Dot Spin | 95 |
| Cyan Dot Spin | 96 |
| Purple Dot Spin | 97 |
| White Dot Spin | 98 |
| Red Segment Spin | 99 |
| Green Segment Spin | 100 |
| Blue Segment Spin | 101 |
| Yellow Segment Spin | 102 |
| Cyan Segment Spin | 103 |
| Purple Segment Spin | 104 |
| White Segment Spin | 105 |
| Red Green Gradual Snake | 106 |
| Red Blue Gradual Snake | 107 |
| Red Yellow Gradual Snake | 108 |
| Red Cyan Gradual Snake | 109 |
| Red Purple Gradual Snake | 110 |
| Red White Gradual Snake | 111 |
| Green Blue Gradual Snake | 112 |
| Green Yellow Gradual Snake | 113 |
| Green Cyan Gradual Snake | 114 |
| Green Purple Gradual Snake | 115 |
| Green White Gradual Snake | 116 |
| Blue Yellow Gradual Snake | 117 |
| Blue Cyan Gradual Snake | 118 |
| Blue Purple Gradual Snake | 119 |
| Blue White Gradual Snake | 120 |
| Yellow Cyan Gradual Snake | 121 |
| Yellow Purple Gradual Snake | 122 |
| Yellow White Gradual Snake | 123 |
| Cyan Purple Gradual Snake | 124 |
| Cyan White Gradual Snake | 125 |
| Purple White Gradual Snake | 126 |
| Red White Blank Snake | 127 |
| Green White Blank Snake | 128 |
| Blue White Blank Snake | 129 |
| Yellow White Blank Snake | 130 |
| Cyan White Blank Snake | 131 |
| Purple White Blank Snake | 132 |
| Green Yellow White Snake | 133 |
| Red Green White Snake | 134 |
| Red Yellow Snake | 135 |
| Red White Snake | 136 |
| Green White Snake | 137 |
| Red Stars | 138 |
| Green Stars | 139 |
| Blue Stars | 140 |
| Yellow Stars | 141 |
| Cyan Stars | 142 |
| Purple Stars | 143 |
| White Stars | 144 |
| Red Background Stars | 145 |
| Green Background Stars | 146 |
| Blue Background Stars | 147 |
| Yellow Background Stars | 148 |
| Cyan Background Stars | 149 |
| Purple Background Stars | 150 |
| Red White Background Stars | 151 |
| Green White Background Stars | 152 |
| Blue White Background Stars | 153 |
| Yellow White Background Stars | 154 |
| Cyan White Background Stars | 155 |
| Purple White Background Stars | 156 |
| White White Background Stars | 157 |
| Red Breath | 158 |
| Green Breath | 159 |
| Blue Breath | 160 |
| Yellow Breath | 161 |
| Cyan Breath | 162 |
| Purple Breath | 163 |
| White Breath | 164 |
| Red Yellow Fire | 165 |
| Red Purple Fire | 166 |
| Green Yellow Fire | 167 |
| Green Cyan Fire | 168 |
| Blue Purple Fire | 169 |
| Blue Cyan Fire | 170 |
| Red Strobe | 171 |
| Green Strobe | 172 |
| Blue Strobe | 173 |
| Yellow Strobe | 174 |
| Cyan Strobe | 175 |
| Purple Strobe | 176 |
| White Strobe | 177 |
| Red Blue White Strobe | 178 |
| Full Color Strobe | 179 |

### 5. Custom Effect Modes

| Name | Value |
|------|-------|
| STATIC | 0 |
| CHASE FORWARD | 1 |
| CHASE BACKWARD | 2 |
| CHASE MIDDLE TO OUT | 3 |
| CHASE OUT TO MIDDLE | 4 |
| STARS | 5 |
| BREATH | 6 |
| COMET FORWARD | 7 |
| COMET BACKWARD | 8 |
| COMET MIDDLE TO OUT | 9 |
| COMET OUT TO MIDDLE | 10 |
| WAVE FORWARD | 11 |
| WAVE BACKWARD | 12 |
| WAVE MIDDLE TO OUT | 13 |
| WAVE OUT TO MIDDLE | 14 |
| STROBE | 15 |
| SOLID FADE | 16 |

### 6. Repetition

| Repetition | Value |
|------------|-------|
| Today Only | 0 |
| Everyday | 1 |
| Week Days | 2 |
| Weekend | 3 |
