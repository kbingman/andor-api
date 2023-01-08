https://stackoverflow.com/questions/1109061/insert-on-duplicate-update-in-postgresql/1109198#1109198

# PostgresSQL 

Sequel Query to group JOINS

```sql
SELECT 
    people.id, 
    people.name, 
    json_agg(people_episodes.episode_id)
FROM people
LEFT JOIN people_episodes on (people.id = people_episodes.person_id)
GROUP BY people.id  
```

`json_agg_strict` and `string_agg` also might work