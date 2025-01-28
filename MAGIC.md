# Magic extra javascript stuff

There is some javascript specified in the mkdocs control file
and stored in `zq2/docs/js` which provides some extra facilities:

## Checkpoints

If you write:

```html
<div
 class="zq2_checkpoints"
 list="<somewhere_to_get_a_list_from>"
 api="API entrypoint"
 number=<nr>
/>
```

Then `checkpoints.js` will generate a table inside your div of the last `nr` checkpoints.
