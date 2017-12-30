# My Website

A simple website to help make my life easier.


## Overall Website Structure

- `/times` - Overall timesheet tracker for the logged in user (shows all 
  available entries and time slices)
  - `/times/entry` - endpoints corresponding to timesheet entries
    - `/new` - Create a new timesheet entry
    - `/:n` - Show the `n`'th timesheet entry
    - `/:n/edit` - Edit the `n`'th timesheet entry
  - `/times/slice` - endpoints corresponding to time slices (a collection of 
    entries between two points in time)
    - `/new` - Create a new unique *time slice*
    - `/:hash` - Show the *time slice* with the corresponding `hash`
    - `/:hash/edit` - Edit the *time slice* with the corresponding `hash`
- `/resume` - My resume
- `/login` - Allows a user to log into the site
- `/logout` - Log out
- `/admin` - The admin area (for managing users, jobs, and backups)


## Getting Started