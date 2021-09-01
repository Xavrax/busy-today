# Busy-Today Gate

## What is it?

This service is a simple gate for [Busy-Today](../../README.md). It can be requested
by calling an HTTP calls with [REST API](open_api.yaml) what gives a possibility to 
communicate with the system. 

Default paths provided by binary:
- `/days` - responds with all stored days with people that declared their availability 


## Examples
### /days

When service gets a `GET` request on a `/days` endpoint, it should respond with 
`200 OK` with an example body:
```json5
{
  "days": [ // array of days
    {
      "date": "2021-08-29T00:00:00Z", // date of the day
      "people": [ // array of people that declared their availability 
        {
          "name": "Xavrax", // person's name
          "declaration": "free" // availability
        },
        {
          "name": "Other",
          "declaration": "busy"
        }
      ]
    },
    {
      "date": "2021-08-30T00:00:00Z"
    }
  ]
}
```

## Configuration

All necessary information can be found by calling service with `--help` flag