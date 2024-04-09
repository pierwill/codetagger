Write a script that inspects the content of pages in a given docs repository. If a page contains:

- `tabs-selector:: drivers`
- `tabs-drivers::`

Or if the page contains an include from this list: https://docs.google.com/spreadsheets/d/12WFKaS-pb_h6Joz_DIa-07mNzpJI32KygJvypGovU8o/edit#gid=947096571

Do both of the following:


## 1
Add a facet directive with a programmingLanguage attribute with values that correspond to all code samples on the page.
(I don’t think any of these pages have programmingLanguage facets, so I think you can ignore that case for now. )
Example:

```
.. facet::
:name: programmingLanguage
:values: shell, csharp, javascript/typescript
```

## 2 
Add the following keywords to the `keywords` meta directive or create it and add them if it doesn’t exist:

code example

Each programming language used on the page

```
.. meta::
:keywords: code example, node.js
```

See https://wiki.corp.mongodb.com/display/DE/FAQ%3A+Taxonomy+Tags for the
values we must use for each programming language.

I think we want the script to be run against one repository at a time.
It might be useful to assume that this will be run from a repository’s
root directory, so perhaps start looking for files inside of source/. We
should open pull requests against each repository and allow members of
the owning team to review, approve, and merge.

Identifying pages that contain other types of code examples are out of
scope of this request.

The taxonomy project deals only with the master / main / current branch, so no need to backport the result of this script to other versions. 
