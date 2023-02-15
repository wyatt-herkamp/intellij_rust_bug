# Sea_ORM Function AutoComplete only available in current crate


## Project Layout
We have our main crate and within the crate. I have a second crate called entities. 

Entities is just all database entities relationship and some helping functions

Then our main crate is we have a lib.rs to show the issue. 

This is layout is copied from another project I have. 


## Issue
As you can see inside the lib.rs the text highlighting is not working 
![img.png](img.png)
The autocomplete is not working for the functions derived from sea_orm in the other crate
![img_1.png](img_1.png)

If we go into the database_helpers module inside the entities crate. All the features are working. 
![img_2.png](img_2.png)

The text highlighting and autocomplete is working for functions you made in the entities crate.
![img_3.png](img_3.png)