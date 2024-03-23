package db

import (
	"context"

	"go.opencensus.io/trace"
)

const DefaultSamplingProbability = 1e-4

func NewTrace(prefix string, actual Database) Database {
	return &profilingDatabase{prefix, actual, DefaultSamplingProbability}
}

func NewTraceSpan(ctx context.Context, prefix, suffix string) (context.Context, TraceSpan) {
	spanName := prefix + "::" + suffix
	ctx, span := trace.StartSpan(ctx, spanName)
	return ctx, TraceSpan{span}
}

func NewTraceWithSampling(prefix string, suffix string, samplingProbability float64) (context.Context, TraceSpan) {
	spanName := prefix + "::" + suffix
	ctx, span := trace.StartSpan(context.Background(), spanName, trace.WithSampler(trace.AlwaysSample()))
	return ctx, TraceSpan{Span: span}
}

type profilingDatabase struct {
	prefix                     string
	actual                     Database
	defaultSamplingProbability float64
}

// DeleteUser implements Database.
func (p *profilingDatabase) DeleteUser(ctx context.Context, id string) error {
	panic("unimplemented")
}

type TraceSpan struct {
	Span *trace.Span
}

type profilerSpan struct {
	TraceSpan
}

func newProfilerSpan(ctx context.Context, prefix, name string, samplingProbability float64) (context.Context, profilerSpan) {
	ctx, span := NewTraceWithSampling(prefix, name, samplingProbability)
	return ctx, profilerSpan{span}
}

func (ts *TraceSpan) End() {
	ts.Span.End()
}

func (ts *TraceSpan) AddAttribute(key, value string) {
	ts.Span.AddAttributes(trace.StringAttribute(key, value))
}

func (ps *profilerSpan) addUserEmail(ctx context.Context, email string) {
	ps.AddAttribute("email", email)
}

func (ps *profilerSpan) addUserID(ctx context.Context, id string) {
	ps.AddAttribute("id", id)
}

// UserByEmail implements Database.
func (p *profilingDatabase) UserByEmail(ctx context.Context, email string) (*User, error) {
	ctx, span := newProfilerSpan(ctx, p.prefix, "UserByEmail", p.defaultSamplingProbability/2)
	defer span.End()
	span.addUserEmail(ctx, email)

	return p.actual.UserByEmail(ctx, email)
}

// UserByID implements Database.
func (p *profilingDatabase) UserByID(ctx context.Context, id string) (*User, error) {
	ctx, span := newProfilerSpan(ctx, p.prefix, "UserByID", p.defaultSamplingProbability/2)
	defer span.End()

	return p.actual.UserByID(ctx, id)
}
