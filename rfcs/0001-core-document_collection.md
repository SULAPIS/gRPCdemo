# Core Document collection

- Feature Name: core-document_collection
- Start Date: 2023-11-16

## Summary

A service that collects documents from stores and shows them to the admin.

## Motivation

To simplify the process of collecting documents.
The original process was that the admin had to go to collect the documents from the stores manually.

## Guide-level explanation

### Service interface

We would use gRPC as a service interface. Below is the proto definition.

```proto
message Document {
    string id = 1;
    string user_id = 2;

    repeated DocumentItem items = 3;

    google.protobuf.Timestamp created_at = 4;
    google.protobuf.Timestamp updated_at = 5;
}

message DocumentItem{
    string name = 1;
    repeated Item1 items = 2;
    repeated Item0 items = 3;
}

message Item1{
    string name = 1;
    repeated ItemLevel0 items = 2;
}

message Item0{
    string name = 1;
    int32 quantity = 2;
    float price = 3;
}

service DocumentCollection {
    rpc get(GetRequest) returns (GetResponse);
    rpc query(QueryRequest) returns (stream Document);
    rpc create(CreateRequest) returns (CreateResponse);
    rpc update(UpdateRequest) returns (UpdateResponse);
    rpc delete(DeleteRequest) returns (DeleteResponse);
}

message GetRequest {
    string id = 1;
}

message GetResponse {
    Document document = 1;
}

message QueryRequest {
    string user_id = 1;
    google.protobuf.Timestamp start = 2;
    google.protobuf.Timestamp end = 3;
}

message CreateRequest {
    Document document = 1;
}

message CreateResponse {
    Document document = 1;
}

message UpdateRequest {
    Document document = 1;
}

message UpdateResponse {
    Document document = 1;
}

message DeleteRequest {
    string id = 1;
}

message DeleteResponse {
    Document document = 1;
}
```

### Database schema

We would use PostgreSQL as a database. Below is the schema.

```sql
CREATE SCHEMA dc;
CREATE TYPE dc.item0 AS (
    name TEXT,
    quantity INT,
    price PRICE
);

CREATE TYPE dc.item1 AS (
    name TEXT,
    items dc.item0[]
);

CREATE TYPE dc.document_item AS (
    name TEXT,
    items1 dc.item1[],
    items0 dc.item0[]
);

CREATE TABLE dc.documents (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    items dc.document_item[] NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL,

    CONSTRAINT documents_pk PRIMARY KEY (id),
);

CREATE INDEX documents_user_id ON dc.documents (user_id);

CREATE OR REPLACE FUNCTION dc.update_updated_at()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER documents_updated_at
    BEFORE UPDATE ON dc.documents
    FOR EACH ROW
    EXECUTE PROCEDURE dc.update_updated_at();
```

## Reference-level explanation

This is the technical portion of the RFC. Explain the design in sufficient detail that:

- Its interaction with other features is clear.
- It is reasonably clear how the feature would be implemented.
- Corner cases are dissected by example.

The section should return to the examples given in the previous section, and explain more fully how the detailed proposal makes those examples work.

## Drawbacks

Why should we *not* do this?

## Rationale and alternatives

- Why is this design the best in the space of possible designs?
- What other designs have been considered and what is the rationale for not choosing them?
- What is the impact of not doing this?

## Prior art

Discuss prior art, both the good and the bad, in relation to this proposal.
A few examples of what this can include are:

- For language, library, cargo, tools, and compiler proposals: Does this feature exist in other programming languages and what experience have their community had?
- For community proposals: Is this done by some other community and what were their experiences with it?
- For other teams: What lessons can we learn from what other communities have done here?
- Papers: Are there any published papers or great posts that discuss this? If you have some relevant papers to refer to, this can serve as a more detailed theoretical background.

This section is intended to encourage you as an author to think about the lessons from other languages, provide readers of your RFC with a fuller picture.
If there is no prior art, that is fine - your ideas are interesting to us whether they are brand new or if it is an adaptation from other languages.

Note that while precedent set by other languages is some motivation, it does not on its own motivate an RFC.
Please also take into consideration that rust sometimes intentionally diverges from common language features.

## Unresolved questions

- What parts of the design do you expect to resolve through the RFC process before this gets merged?
- What parts of the design do you expect to resolve through the implementation of this feature before stabilization?
- What related issues do you consider out of scope for this RFC that could be addressed in the future independently of the solution that comes out of this RFC?

## Future possibilities

Think about what the natural extension and evolution of your proposal would
be and how it would affect the language and project as a whole in a holistic
way. Try to use this section as a tool to more fully consider all possible
interactions with the project and language in your proposal.
Also consider how this all fits into the roadmap for the project
and of the relevant sub-team.

This is also a good place to "dump ideas", if they are out of scope for the
RFC you are writing but otherwise related.

If you have tried and cannot think of any future possibilities,
you may simply state that you cannot think of anything.

Note that having something written down in the future-possibilities section
is not a reason to accept the current or a future RFC; such notes should be
in the section on motivation or rationale in this or subsequent RFCs.
The section merely provides additional information.
