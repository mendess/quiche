searchState.loadedDescShard("tokio_quiche", 0, "Bridging the gap between quiche and tokio.\nA trait to implement an application served over QUIC.\nGeneric thread-safe boxed error.\nA customizable generator to derive and verify QUIC …\nContains the error value\nA QUIC connection that has not performed a handshake yet.\nContains the success value\nMetadata about an established QUIC connection.\nA stream of accepted <code>InitialQuicConnection</code>s from a <code>listen</code> …\nResult alias based on <code>BoxError</code> for this crate.\nA handle to the <code>QuicAuditStats</code> for this connection.\nA handle to the <code>QuicAuditStats</code> for this connection.\nPooled buffers for zero-copy packet handling.\nA borrowed buffer for the worker to write outbound packets …\nReturns the argument unchanged.\nReturns the argument unchanged.\nPerforms the QUIC handshake in a separate tokio task and …\nCreates a future to drive the connection’s handshake.\nHTTP/3 integrations for tokio-quiche.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nStarts listening for inbound QUIC connections on the given …\nStarts listening for inbound QUIC connections on the given …\nThe local address this connection listens on.\nThe local address this connection listens on.\nMetrics collected across QUIC connections.\nCreates a new <code>ConnectionId</code> according to the generator’s …\nCallback to inspect the result of the worker task, before …\nCallback to inspect the result of the worker task, before …\nCallback to customize the <code>ApplicationOverQuic</code> after the …\nThe remote address for this connection.\nThe remote address for this connection.\nProcesses data received on the connection.\nAdds data to be sent on the connection.\n<code>async</code>-ified QUIC connections powered by quiche.\nResumes a QUIC connection which was paused after a …\nThe QUIC source connection ID used by this connection.\nConfiguration for QUIC connections.\nDetermines whether the application’s methods will be …\nNetwork socket utilities and wrappers.\nDrives a QUIC connection from handshake to close in …\nA handle to the <code>QuicConnectionStats</code> for this connection.\nA handle to the <code>QuicConnectionStats</code> for this connection.\nVerifies whether <code>cid</code> was generated by this …\nWaits for an event to trigger the next iteration of the …\nHandle to the crate’s static buffer pools.\nThe maximum size of the buffers in the generic pool. …\nThe maximum size of the buffers in the datagram pool.\nA pooled byte buffer to pass stream data around without …\nA pooled byte buffer to pass datagrams around without …\nA pooled, splittable byte buffer for zero-copy <code>quiche</code> …\nFetches a <code>PooledBuf</code> from the generic pool and initializes …\nFetches a <code>PooledDgram</code> from the datagram pool and …\nAdds <code>dgram</code> to the datagram pool without copying it.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates an empty <code>PooledBuf</code> which is not taken from the …\nCreates an empty <code>PooledDgram</code> which is not taken from the …\nFetches a <code>MAX_BUF_SIZE</code> sized <code>PooledBuf</code> from the generic …\nFetches a <code>MAX_DATAGRAM_SIZE</code> sized <code>PooledDgram</code> from the …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nStream-level HTTP/3 audit statistics recorded by H3Driver.\nThe number of bytes received over the stream.\nThe number of bytes sent over the stream.\nAn <code>ApplicationOverQuic</code> to build clients and servers on top …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nA RESET_STREAM error code received from the peer.\nA STOP_SENDING error code received from the peer.\nStream FIN received from the peer.\nA RESET_STREAM error code sent to the peer.\nA STOP_SENDING error code sent to the peer.\nStream FIN sent to the peer.\nConfiguration for HTTP/3 connections.\nThe stream ID of this session.\nResponse body/CONNECT downstream data plus FIN flag.\nRequest body/CONNECT upstream data plus FIN flag.\nBody data has been received over a stream.\nReceives <code>ClientH3Event</code>s from a ClientH3Driver. This is the …\nCommands accepted by ClientH3Driver.\nThe H3Controller type paired with ClientH3Driver. See …\nAn H3Driver for a client-side HTTP/3 connection. See …\nEvents produced by ClientH3Driver.\nSend a new HTTP request over the <code>quiche::h3::Connection</code>. …\nA RequestSender to send HTTP requests over a ClientH3Driver…\nThe connection has irrecoverably errored and is shutting …\nThe connection has been shutdown, optionally due to an …\nThe controller task was shut down and is no longer …\nCONNECT-UDP (DATAGRAM) downstream data plus flow ID.\nCONNECT-UDP (DATAGRAM) upstream data.\nDATAGRAM flow explicitly closed.\nReceived a GOAWAY frame from the peer.\nSend a GOAWAY frame to the peer to initiate a graceful …\nOther error at the connection, but not stream level.\n<code>H3Command</code>s are sent by the H3Controller to alter the …\nThe error type used internally in H3Driver.\nInterface to communicate with a paired H3Driver.\nA ready-made <code>ApplicationOverQuic</code> which can handle HTTP/3 …\n<code>H3Event</code>s are produced by an H3Driver to describe HTTP/3 …\nResponse headers to be sent to the peer.\nAn <code>InboundFrame</code> is a data frame that was received from the …\nUsed by a local task to receive <code>InboundFrame</code>s (data) on …\nHTTP/3 headers that were received on a stream.\nA HEADERS frame was received on the given stream. This is …\nA SETTINGS frame was received.\nAn HTTP request sent using a ClientRequestSender to the …\nA DATAGRAM flow was created and associated with the given …\nHeaders for the request with the given <code>request_id</code> were …\nReceived data for a stream that was closed or never opened.\nAn <code>OutboundFrame</code> is a data frame that should be sent from …\nUsed by a local task to send <code>OutboundFrame</code>s to a peer on …\nAn error encountered when serving the request. Stream …\nThe server’s post-accept timeout was hit. The timeout …\nA connection-level command that executes directly on the …\nSends <code>H3Command</code>s to an H3Driver. The sender is typed and …\nA RST_STREAM frame was seen on the given <code>stream_id</code>. The …\nReceives <code>ServerH3Event</code>s from a ServerH3Driver. This is the …\nCommands accepted by ServerH3Driver.\nThe H3Controller type paired with ServerH3Driver. See …\nAn H3Driver for a server-side HTTP/3 connection. See …\nEvents produced by ServerH3Driver.\nThe stream has been closed. This is used to signal stream …\nCreates a body frame with the provided buffer.\nA sender to pass the request’s <code>OutboundFrameSender</code> to …\nCreates a <code>QuicCommand</code> sender for the paired H3Driver.\nWrapper for running HTTP/3 connections.\nGets a mut reference to the <code>H3Event</code> receiver for the paired\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nHandle to the <code>H3AuditStats</code> for the message’s stream.\nThe <code>h3::Header</code>s that make up this request.\nThe actual <code>h3::Header</code>s which were received.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nBuilds a new H3Driver and an associated H3Controller.\nReports connection-level error metrics and forwards …\nPoll the underlying <code>quiche::h3::Connection</code> for …\nWrite as much data as possible into the …\nWhether there is a body associated with the incoming …\nAn <code>InboundFrameStream</code> of body data received from the peer.\nA user-defined identifier to match …\nCreates a <code>NewClientRequest</code> sender for the paired …\nCreates a <code>NewClientRequest</code> sender for the paired …\nSend a request to the H3Driver. This can only fail if the …\nAn <code>OutboundFrameSender</code> for streaming body data to the …\nSends a GOAWAY frame to initiate a graceful connection …\nStream ID of the frame.\nTakes the <code>H3Event</code> receiver for the paired H3Driver.\nWait for incoming data from the H3Controller. The next …\nWhether the stream is finished and won’t yield any more …\nFlow ID of the new flow.\nNumber of bytes received.\nAn <code>InboundFrameStream</code> for receiving datagrams from the …\nAn <code>OutboundFrameSender</code> for transmitting datagrams to the …\nRaw HTTP/3 setting pairs, in the order received from the …\nStream ID of the body data.\nA wrapper for an h3-driven QuicConnection together with …\nThe QuicConnection’s audit stats.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nThe local address this connection listens on.\nBundles <code>quic_connection</code> and <code>h3_controller</code> into a new …\nThe remote address for this connection.\nThe QuicConnection’s source connection ID.\nThe QuicConnection’s <code>quiche</code> stats.\nUnified configuration parameters for H3Drivers.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nMaximum size of a single HEADERS frame, in bytes.\nMaximum number of requests a ServerH3Driver allows per …\nTimeout between starting the QUIC handshake and receiving …\nUpper bound on the number of streams that can be blocked …\nMaximum value the QPACK encoder is permitted to set for …\nStandard implementation of <code>Metrics</code> using …\nTrait to direct the metrics emitted by the crate to a …\nNumber of accepted QUIC Initial packets\nNumber of QUIC connections currently in memory\nNumber of accepted QUIC Initial packets using expensive …\nNumber of QUIC packets received but not associated with an …\nNumber of failed quic handshakes\nReturns the argument unchanged.\nOverhead of QUIC handshake processing stage\nCalls <code>U::from(self)</code>.\nNumber of QUIC packets received where the CID could not be …\nLabels for crate metrics.\nNumber of HTTP/3 connection closures generated locally\nNumber of QUIC connection closures generated locally\nThe highest utilized bandwidh reported during the lifetime …\nThe highest momentary loss reported during the lifetime of …\nMaximum number of writable QUIC streams in a connection\nNumber of HTTP/3 connection closures generated by peer\nNumber of QUIC connection closures generated by peer\nNumber of QUIC packets received but not associated with an …\nHistogram of task poll durations\nHistogram of task poll durations\nHistogram of task schedule delays\nHistogram of task schedule delays\nHelps us get a rough idea of if our waker is causing …\nHelps us get a rough idea of if our waker is causing …\nInstrumentation and metrics for spawned tokio tasks.\nNumber of UDP packets dropped when receiving\nCombined utilized bandwidth of all open connections (max …\nNumber of error and partial writes while sending QUIC …\nHTTP/3 error code (from IANA registry).\nCategory of error that caused the QUIC handshake to fail.\nQUIC error code (from IANA registry).\nType of handshake latency that was measured by a metric.\nReason why a QUIC Initial was discarded by the packet …\nType of UDP <code>send(2)</code> error observed.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSpawn a potentially instrumented task.\nSpawn a potentially instrumented, long-lived task. …\nClose the connection with the given parameters.\nThe connection was closed while handshaking, for example …\nA set of hooks executed at the level of a …\nParameters to close a quiche::Connection.\nExecute a custom callback on the connection.\nAdditional error types that can occur during a QUIC …\nDetails about a connection’s QUIC handshake.\nA received network packet with additional metadata.\nA command to execute on a quiche::Connection in the …\nWrapper for connection statistics recorded by quiche.\nAlias of quiche::Connection used internally by the crate.\nA <code>ConnectionIdGenerator</code> which creates random 20-byte …\nCollect the current <code>SocketStats</code> from the connection.\nThe configured handshake timeout has expired.\nThe packet’s contents.\nConnects to an HTTP/3 server using <code>socket</code> and the default …\nConnects to a QUIC server using <code>socket</code> and the provided …\nConstructs an optional <code>SslContextBuilder</code>.\nHow long the handshake took to complete.\nThe QUIC or application-level error code to send to the …\nConsume the command and perform its operation on <code>qconn</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nIf set, then <code>buf</code> is a GRO buffer containing multiple …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThe address on which we received the inbound packet.\nSpecific statistics about the connection’s active path.\nThe address that sent the inbound packet.\nHelper to wrap existing quiche::Connections.\nThe reason phrase to send to the peer.\nThe receive timestamp of the packet.\nWhether to send an application close or a regular close to …\nThe time at which the connection was created.\nAggregate connection statistics across all paths.\nPerforms no verification, because this generator can create\nPollable receiver for <code>connection closed</code> notifications from …\nResult of manually wrapping a <code>quiche::Connection</code> in an …\nThe connection wrapper.\nReceiver for <code>connection closed</code> notifications. This fires …\nReturns the argument unchanged.\nReturns the argument unchanged.\nSender for inbound packets on the connection.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPolls to receive a <code>connection closed</code> notification.\nWaits for a <code>connection closed</code> notification.\nReceiver which fires only when its associated sender is …\nWraps an existing <code>quiche::Connection</code> in an …\nTypes of PKI certificates supported by the crate.\nCombined configuration parameters required to establish a …\nHook configuration for use in the QUIC connection …\nQUIC configuration parameters.\nRaw public key TLS certificate.\nTLS credentials to authenticate the endpoint.\nStandard X509 TLS certificate.\nConfigures the list of supported application protocols. …\nForwards <code>quiche</code> logs into the logging system currently …\nCongestion control algorithm to use.\nPath to the endpoint’s TLS certificate.\nMax queue length for received DATAGRAM frames. Defaults to …\nMax queue length for sending DATAGRAM frames. Defaults to …\nConfigures whether the local endpoint supports active …\nWhether to validate client IPs in QUIC initials.\nConfigures whether to enable DATAGRAM frame support. H3 …\nOptionally enables expensive versions of the …\nWhether to use HyStart++ (only with <code>cubic</code> and <code>reno</code> CC). …\nOptionally enables pacing for outgoing packets.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nA timeout for the QUIC handshake, in milliseconds. …\nHooks to use for the connection.\nSets the <code>initial_max_data</code> transport parameter. Defaults to …\nSets the <code>initial_max_stream_data_bidi_local</code> transport …\nSets the <code>initial_max_stream_data_bidi_remote</code> transport …\nSets the <code>initial_max_stream_data_uni</code> transport parameter. …\nSets the <code>initial_max_streams_bidi</code> transport parameter. …\nSets the <code>initial_max_streams_uni</code> transport parameter. …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nPath to a file in which TLS secrets will be logged in …\n<code>cert</code>’s PKI certificate type.\nThe maximum number of newly-created connections that will …\nConfigures the max idle timeout of the connection in …\nSets the maximum incoming UDP payload size. Defaults to …\nSets the maximum outgoing UDP payload size. Defaults to …\nCreates <code>ConnectionParams</code> for a QUIC client. Clients may …\nCreates <code>ConnectionParams</code> for a QUIC server. Servers should …\nPath to the endpoint’s private key.\nPath to a directory where QLOG files will be saved.\nQUIC connection settings.\nOptional TLS credentials to authenticate with.\nA type-erased variant of <code>Socket</code> with boxed <code>Tx</code> and <code>Rx</code> …\nWrapper around a <code>UdpSocket</code> for server-side QUIC …\nA connected datagram socket with separate <code>send</code> and <code>recv</code> …\nIndicators of sockopts configured for a socket.\nBuilder to enable Linux sockopts which improve QUIC …\nTests whether <code>IP_FREEBIND</code> or <code>IP_TRANSPARENT</code> are enabled …\nTries to enable all supported sockopts and returns …\nTries to enable all sockopts supported by the crate for …\nChecks whether both <code>send</code> and <code>recv</code> refer to the same …\nThe <code>SocketCapabilities</code> to use for this socket.\nEnables <code>SO_RXQ_OVFL</code>, which reports dropped packets due to …\nConsumes the builder and returns the configured …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCreates a <code>Socket</code> from a <code>UdpSocket</code> by wrapping the file …\nEnables <code>UDP_GRO</code>, a generic receive offload (GRO).\nEnables <code>UDP_SEGMENT</code>, a generic segmentation offload (GSO).\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nSets <code>IP_MTU_DISCOVER</code>, to <code>IP_PMTUDISC_PROBE</code>, which disables …\nEnables <code>IP_PKTINFO</code> to control the source IP in outbound …\nEnables <code>IP_RECVORIGDSTADDR</code>, which reports each packet’s …\nSets <code>IPV6_MTU_DISCOVER</code>, to <code>IPV6_PMTUDISC_PROBE</code>, which …\nEnables <code>IPV6_RECVPKTINFO</code> to control the source IP in …\nEnables <code>IPV6_RECVORIGDSTADDR</code>, which reports each packet’…\nThe address of the local endpoint.\nThe address of the local endpoint.\nCreates a new sockopt builder for <code>socket</code>.\nThe address of the remote endpoint.\nThe address of the remote endpoint.\nThe receiving half of the connection. This is generally …\nThe receiving half of the connection. This is generally …\nEnables <code>SO_TIMESTAMPNS</code>, which records a wall-clock …\nThe sending half of the connection. This generally …\nThe sending half of the connection. This generally …\nThe wrapped tokio socket.\nAn opaque value that is later passed to the …\nEnables <code>SO_TXTIME</code> to control packet transmit timestamps …")