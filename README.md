# SkyScanner-Scanner



# How do I choose flights?
Skyscanner makes it possible to find low-cost flights to almost any destination. However, it's search interface often makes it hard for me to locate flights that are suitable for me. 

## Criteria

### Dates of flights
Often I do not have a fixed schedule in which to fly but would like to travel within a certain period, e.g. if I would like to snowboard in New Zealand then I need to travel in the winter, perhaps June to August.

### Time spent within destination
I need to be able to spend a decent time within a country. It doesn't make sense to spend 48 hours flying from Sydney to London just to spend 5 days within a country.

### Cost
I will almost always have an upper limit on what I am willing or able to pay for a flight.

### Flight time
While I'm willing to pay less for a flight that may be less direct, I don't want to spend 48 hours in the air.

### Holiday time spent
I have a full time job and a limited amount of holiday days to sepnd each year. Getting the most time away for the least amount of holiday time used is very important. Holiday time is spent when:
- I am contracted to work (9-5, Mon-Fri)
- It is not a public holiday (in the country I am currently working)

While my model of holiday time may differ significantly to others, it should be possible to model other these.

It is useful to know to holiday time cost of the flights as well as the total holiday time cost of the trip (flights + stay).

### Quality of airline
A higher quality airline often means a more comfortable and safe flight. The SKYTRAX rating (or a similar service) of an airline influences what flight I'll choose. 

## 



# Reporting flights found

## Model
- departure
    - airport
    - date
- arrival 
    - airport
    - date
- duration_days
- holiday_days


- destination
- 
Store in elasticsearch  
- Index per day
- Can search for particular flights found
- Can sort by criteria
- Can filter on properties (cost less for example)
Can sort on criteria

Should alert if a new cheapest flight has been found
